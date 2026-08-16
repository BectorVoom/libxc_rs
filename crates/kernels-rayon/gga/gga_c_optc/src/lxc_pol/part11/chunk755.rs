//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 755/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk755(t1382: f64, t7467: f64, t7481: f64, t311: f64, t7856: f64, t10: f64, t2595: f64, t896: f64, t2673: f64, t2638: f64, t330: f64, t1378: f64, t530: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10894 = t7467 * t1382;
    let t10917 = t7481 * t1382;
    let t10935 = t311 * t7856;
    let t10959 = t10 * t2595;
    let t10975 = t896 * t1382;
    let t10976 = t10975 * t2673;
    let t10990 = t2638 * t311;
    let t10991 = t330 * t10990;
    let t11007 = t530 * t1378;
    (t10894, t10917, t10935, t10959, t10975, t10976, t10990, t10991, t11007)
}
