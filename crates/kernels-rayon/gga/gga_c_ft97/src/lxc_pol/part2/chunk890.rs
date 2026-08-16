//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 890/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk890(t13309: f64, t2594: f64, t446: f64, t10024: f64, t13315: f64, t13320: f64, t3281: f64, t13324: f64, t2354: f64, t1882: f64, t3696: f64, t3701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13797 = t2594 * t13309;
    let t13798 = t446 * t13797;
    let t13800 = t10024 * t13315;
    let t13801 = t446 * t13800;
    let t13803 = t2594 * t13320;
    let t13804 = t3281 * t13803;
    let t13806 = t2354 * t13324;
    let t13807 = t446 * t13806;
    let t13809 = t1882 * t3696;
    let t13810 = 2.0_f64 / 27.0_f64 * t13809;
    let t13811 = t1882 * t3701;
    (t13798, t13801, t13804, t13807, t13809, t13810, t13811)
}
