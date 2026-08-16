//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 769/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk769(t21296: f64, t2379: f64, t5019: f64, t7853: f64, t200: f64, t21130: f64, t1127: f64, t4978: f64, t680: f64, t17964: f64, t17993: f64, t21277: f64, t21282: f64, t21285: f64, t21289: f64, t21292: f64, t21297: f64, t2387: f64, t2394: f64, t3759: f64, t6757: f64, t678: f64, t807: f64, t9524: f64, t9609: f64) -> (f64, f64, f64, f64) {
    let t21300 = t2379 * t21296;
    let t21306 = t7853 * t5019;
    let t21309 = t21130 * t200;
    let t21319 = t4978 * t1127;
    let t21320 = t680 * t21319;
    let t21323 = -0.69764702839313376e-1_f64 * t3759 * t680 * t21277 - 0.69764702839313376e-1_f64 * t3759 * t21282 + 0.11619434043764639964e-2_f64 * t3759 * t2379 * t21285 - 0.58097170218823199822e-3_f64 * t2387 * t21289 + 0.139529405678626752e0_f64 * t17964 * t6757 * t21292 + 0.69764702839313376e-2_f64 * t678 * t21297 + 0.58097170218823199823e-3_f64 * t678 * t21300 + 0.139529405678626752e-1_f64 * t3759 * t2394 * t21285 - 0.7112856777411015585e-1_f64 * t17993 * t21306 - 0.32253953169881963531e-5_f64 * t678 * t807 * t21309 - 0.279058811357253504e-2_f64 * t678 * t9609 * t21309 - 0.11619434043764639964e-3_f64 * t678 * t9524 * t21309 + 0.34882351419656688e-1_f64 * t2387 * t21320;
    (t21306, t21309, t21319, t21323)
}
