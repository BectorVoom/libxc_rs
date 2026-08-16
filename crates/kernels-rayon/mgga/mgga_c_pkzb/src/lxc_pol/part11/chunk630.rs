//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 630/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk630(t218: f64, t219: f64, t3542: f64, t208: f64, t3515: f64, t1870: f64, t1881: f64, t2730: f64, t2772: f64, t3517: f64, t3529: f64, t3533: f64, t3537: f64, t3539: f64) -> (f64, f64, f64, f64) {
    let t3544 = t218 * t219 * t3542;
    let t3546 = t208 * t3515;
    let t3548 = t218 * t219 * t3546;
    let t3550 = -0.9494625e0_f64 * t3529 + 0.1898925e1_f64 * t3533 + t1870 - 0.59793333333333333334e0_f64 * t2730 + 0.8969e0_f64 * t3517 + 0.15358125e0_f64 * t3537 + 0.3071625e0_f64 * t3539 + t1881 - 0.32862666666666666666e0_f64 * t2772 + 0.24647e0_f64 * t3544 + 0.24647e0_f64 * t3548;
    (t3544, t3546, t3548, t3550)
}
