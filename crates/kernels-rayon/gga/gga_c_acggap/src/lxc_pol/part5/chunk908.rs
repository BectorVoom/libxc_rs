//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 908/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk908(t13726: f64, t3114: f64, t576: f64, t3117: f64, t138: f64, t3152: f64, t134: f64, t3159: f64, t7322: f64, t3140: f64, t347: f64, t1056: f64, t3143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13727 = 0.142625e1_f64 * t13726;
    let t13728 = t576 * t3114;
    let t13729 = t13728 * t3117;
    let t13736 = t3152 * t138;
    let t13737 = 0.16481111111111111111e2_f64 * t13736;
    let t13745 = t7322 * t134 * t3159;
    let t13746 = 0.163e2_f64 * t13745;
    let t13747 = t3140 * t134;
    let t13748 = t13747 * t347;
    let t13750 = t3143 * t1056;
    (t13727, t13728, t13729, t13736, t13737, t13745, t13746, t13747, t13748, t13750)
}
