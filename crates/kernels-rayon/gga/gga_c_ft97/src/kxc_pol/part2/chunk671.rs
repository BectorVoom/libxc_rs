//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 671/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk671(t1771: f64, t745: f64, t241: f64, t9567: f64, t1775: f64, t2503: f64, t2489: f64, t2508: f64, t458: f64, t9698: f64, t259: f64, t89: f64, t9555: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9936 = t1771 * t745;
    let t9952 = t9567 * t241;
    let t9958 = t1775 * t2503;
    let t9960 = t1775 * t2489;
    let t9962 = t458 * t2508;
    let t9972 = 28.0_f64 / 81.0_f64 * t9698;
    let t9982 = 28.0_f64 / 81.0_f64 * t89 * t9555 * t259;
    (t9936, t9952, t9958, t9960, t9962, t9972, t9982)
}
