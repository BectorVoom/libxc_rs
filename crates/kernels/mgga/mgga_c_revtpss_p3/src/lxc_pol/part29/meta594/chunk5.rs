//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1994/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1994<F: Float>(t28472: F, t98633: F, t198: F, t206: F, t8019: F, t1940: F, t2257: F, t2403: F, t25208: F, t25211: F, t25452: F, t26425: F, t26585: F, t27169: F, t27402: F, t28291: F, t28460: F, t7428: F, t7432: F, t7787: F, t8020: F, t95527: F, t98694: F, t98699: F, t98702: F, t98713: F, t98716: F, t98764: F, t99558: F) -> (F, F, F) {
    let t102877 = F::cast_from(2.0_f64) * t28472 * t98633;
    let t102888 = t198 * t206 * t8019;
    let t102905 = t1940 * t8020 * t2257 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2403 * t8020 * t25211 - t1940 * t26585 * t27402 - t102877 - t1940 * t7432 * t98702 - F::cast_from(3.0_f64) * t26425 * t98694 - t1940 * t95527 * t7787 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t28472 * t98764 - F::cast_from(3.0_f64) * t102888 * t25208 - F::cast_from(3.0_f64) * t26425 * t99558 + F::cast_from(6.0_f64) * t28291 * t98713 + F::cast_from(6.0_f64) * t28291 * t98716 + F::cast_from(3.0_f64) * t28291 * t98699 + F::cast_from(3.0_f64) * t2403 * t7428 * t27169 - t1940 * t28460 * t25452 / F::cast_from(2.0_f64);
    (t102877, t102888, t102905)
}
