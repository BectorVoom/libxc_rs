//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 505/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk505(t277: f64, t715: f64, t43: f64, t98: f64, t34: f64, t39: f64, t100: f64, t50: f64, t712: f64, t47: f64, t52: f64, t441: f64, t939: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2847 = t715 * t277;
    let t2861 = 1.0_f64 / t98 / t43;
    let t2868 = t34 * t39;
    let t2876 = 1.0_f64 / t100 / t50;
    let t2894 = t712 * t277;
    let t2896 = t43 * t43;
    let t2898 = 1.0_f64 / t47 / t2896;
    let t2908 = t50 * t50;
    let t2910 = 1.0_f64 / t52 / t2908;
    let t2929 = t939 * t441;
    (t2847, t2861, t2868, t2876, t2894, t2898, t2910, t2929)
}
