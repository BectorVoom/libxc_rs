//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2031/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2031<F: Float>(t100858: F, t103553: F, t14749: F, t14767: F, t15071: F, t1544: F, t1583: F, t1940: F, t198: F, t207: F, t2071: F, t2394: F, t2403: F, t26425: F, t26581: F, t26590: F, t28291: F, t2832: F, t28460: F, t4343: F, t4433: F, t4541: F, t61155: F, t61182: F, t63186: F, t7428: F, t7432: F, t8020: F, t892: F, t95527: F, t95964: F, t98759: F, t98786: F) -> F {
    let t103658 = F::cast_from(12.0_f64) * t26425 * t100858 - t1940 * t7432 * t15071 + F::cast_from(6.0_f64) * t2403 * t26590 * t61155 - F::cast_from(6.0_f64) * t4541 * t7432 * t98759 + F::cast_from(12.0_f64) * t4541 * t7428 * t4433 + F::cast_from(6.0_f64) * t4541 * t8020 * t2394 - t1940 * t28460 * t2832 + t198 * t207 * t103553 * t892 + F::cast_from(6.0_f64) * t2403 * t7428 * t4343 - F::cast_from(6.0_f64) * t2403 * t7432 * t61182 - F::cast_from(12.0_f64) * t28291 * t63186 + F::cast_from(12.0_f64) * t4541 * t2071 * t14749 + F::cast_from(6.0_f64) * t4541 * t2071 * t14767 - F::cast_from(6.0_f64) * t1940 * t95964 * t98786 - t1940 * t95527 * t1583 + F::cast_from(3.0_f64) * t2403 * t26581 * t1544;
    t103658
}
