//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 785/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk785(t1168: f64, t5053: f64, t729: f64, t762: f64, t1131: f64, t5147: f64, t5064: f64, t2568: f64, t242: f64, t1091: f64, t5073: f64, t10007: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21486 = t5053 * t1168;
    let t21488 = t729 * t762 * t21486;
    let t21490 = t1131 * t5147;
    let t21492 = t729 * t762 * t21490;
    let t21494 = t5064 * t1131;
    let t21496 = t729 * t2568 * t21494;
    let t21499 = t1168 * t5147;
    let t21500 = t2568 * t21499;
    let t21501 = t242 * t21500;
    let t21504 = t5073 * t1091;
    let t21505 = t10007 * t21504;
    (t21486, t21488, t21490, t21492, t21494, t21496, t21499, t21500, t21501, t21504, t21505)
}
