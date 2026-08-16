//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 716/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk716(t1152: f64, t1771: f64, t2345: f64, t26: f64, t2347: f64, t743: f64, t666: f64, t2360: f64, t1087: f64, t89: f64, t9733: f64, t11401: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13680 = t1771 * t1152;
    let t13682 = t26 * t2345;
    let t13683 = t743 * t2347;
    let t13688 = t26 * t666;
    let t13689 = t743 * t2360;
    let t13722 = t89 * t9733 * t1087;
    let t13723 = 4.0_f64 / 81.0_f64 * t13722;
    let t13730 = t11401 * t665;
    (t13680, t13682, t13683, t13688, t13689, t13722, t13723, t13730)
}
