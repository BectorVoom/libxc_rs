//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1265/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1265(t1307: f64, t28388: f64, t52697: f64, t94626: f64, t98146: f64, t98466: f64, t98637: f64, t98640: f64, t98643: f64, t98646: f64, t98649: f64, t98652: f64, t98653: f64, t98663: f64, t98666: f64) -> f64 {
    let t98668 = 0.185671721767578125e-4_f64 * t28388 * t98146 + 0.22109259259259259258e-2_f64 * t98637 - 0.33163888888888888888e-2_f64 * t98640 + 0.13265555555555555555e-1_f64 * t98643 - 0.88437037037037037034e-2_f64 * t98646 + t98649 + t98652 - 0.92673611111111111113e-3_f64 * t94626 * t98653 * t52697 * t1307 - 0.46336805555555555556e-3_f64 * t94626 * t98466 + 0.66327777777777777776e-2_f64 * t98663 - 0.22109259259259259258e-2_f64 * t98666;
    t98668
}
