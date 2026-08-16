//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2535/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2535(t19856: f64, t225: f64, t366: f64, t15696: f64, t4782: f64, t4787: f64, t1058: f64, t6318: f64, t1053: f64, t6317: f64, t4786: f64, t6096: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19857 = t19856 * t225;
    let t19858 = t19857 * t366;
    let t19861 = t15696 * t4782;
    let t19864 = t15696 * t4787;
    let t19867 = t6318 * t1058;
    let t19869 = t6317 * t1053;
    let t19872 = t6096 * t4786;
    (t19857, t19858, t19861, t19864, t19867, t19869, t19872)
}
