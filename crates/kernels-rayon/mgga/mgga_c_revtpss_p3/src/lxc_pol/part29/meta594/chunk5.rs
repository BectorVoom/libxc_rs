//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1994/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1994(t28472: f64, t98633: f64, t198: f64, t206: f64, t8019: f64, t1940: f64, t2257: f64, t2403: f64, t25208: f64, t25211: f64, t25452: f64, t26425: f64, t26585: f64, t27169: f64, t27402: f64, t28291: f64, t28460: f64, t7428: f64, t7432: f64, t7787: f64, t8020: f64, t95527: f64, t98694: f64, t98699: f64, t98702: f64, t98713: f64, t98716: f64, t98764: f64, t99558: f64) -> (f64, f64, f64) {
    let t102877 = 2.0_f64 * t28472 * t98633;
    let t102888 = t198 * t206 * t8019;
    let t102905 = t1940 * t8020 * t2257 / 2.0_f64 + 3.0_f64 * t2403 * t8020 * t25211 - t1940 * t26585 * t27402 - t102877 - t1940 * t7432 * t98702 - 3.0_f64 * t26425 * t98694 - t1940 * t95527 * t7787 / 2.0_f64 + 2.0_f64 * t28472 * t98764 - 3.0_f64 * t102888 * t25208 - 3.0_f64 * t26425 * t99558 + 6.0_f64 * t28291 * t98713 + 6.0_f64 * t28291 * t98716 + 3.0_f64 * t28291 * t98699 + 3.0_f64 * t2403 * t7428 * t27169 - t1940 * t28460 * t25452 / 2.0_f64;
    (t102877, t102888, t102905)
}
