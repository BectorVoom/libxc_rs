//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 561/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk561(t10773: f64, t2508: f64, t3448: f64, t7137: f64, t3459: f64, t841: f64, t1052: f64, t2728: f64, t1022: f64, t830: f64, t1: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10775 = 0.76905262301422242837e-2_f64 * t2508 * t10773;
    let t10788 = 0.20508069947045931423e-1_f64 * t7137 * t3448;
    let t10802 = t3459 * t841;
    let t10805 = t1052 * t2728;
    let t10809 = t830 * t1022;
    let t10810 = t10809 * t1;
    let t10811 = t787 * t10810;
    (t10775, t10788, t10802, t10805, t10809, t10811)
}
