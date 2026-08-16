//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 554/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk554(t2021: f64, t7491: f64, t511: f64, t892: f64, t504: f64, t880: f64, t2144: f64, t1320: f64, t1322: f64, t1325: f64, t2016: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7492 = t7491 * t2021;
    let t7494 = t892 * t511;
    let t7501 = t504 * t880;
    let t7508 = t504 * t2144;
    let t7551 = t1320 * t1322;
    let t7552 = t7551 * t1325;
    let t7553 = t2016 * t7552;
    (t7492, t7494, t7501, t7508, t7551, t7552, t7553)
}
