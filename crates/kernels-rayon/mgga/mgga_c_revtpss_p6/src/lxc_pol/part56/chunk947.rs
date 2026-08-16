//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 947/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk947(t3140: f64, t860: f64, t8477: f64, t31798: f64, t25386: f64, t31837: f64, t31830: f64, t7002: f64, t93: f64, t1419: f64, t25875: f64, t32275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32425 = t860 * t3140;
    let t32426 = t8477 * t32425;
    let t32463 = t8477 * t31798;
    let t32469 = t25386 * t31837;
    let t32474 = t31830 * t31837;
    let t32655 = t93 * t7002;
    let t32699 = t1419 * t3140;
    let t32700 = t8477 * t32699;
    let t32705 = t25875 * t32275;
    (t32425, t32426, t32463, t32469, t32474, t32655, t32699, t32700, t32705)
}
