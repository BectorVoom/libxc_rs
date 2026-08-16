//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 588/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk588(t1885: f64, t2626: f64, t1820: f64, t1017: f64, t1802: f64, t610: f64, t587: f64, t597: f64, t995: f64, t1036: f64, t1630: f64, t639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2627 = t1885 * t2626;
    let t2629 = 4.0_f64 / 15.0_f64 * t1820 * t2627;
    let t2630 = t1802 * t1017;
    let t2631 = t2630 * t610;
    let t2632 = t1885 * t2631;
    let t2634 = 4.0_f64 / 15.0_f64 * t587 * t2632;
    let t2635 = t597 * t995;
    let t2636 = t2635 * t610;
    let t2637 = t1885 * t2636;
    let t2639 = 4.0_f64 / 15.0_f64 * t1820 * t2637;
    let t2640 = t1630 * t1036;
    let t2641 = t639 * t2640;
    (t2627, t2629, t2630, t2631, t2632, t2634, t2635, t2636, t2637, t2639, t2640, t2641)
}
