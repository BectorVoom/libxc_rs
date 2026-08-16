//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 521/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk521(t5384: f64, t5410: f64, t5442: f64, t5749: f64, t109: f64, t574: f64, t934: f64, t352: f64, t570: f64) -> (f64, f64, f64, f64) {
    let t5751 = t5384 + t5410 + t5442 + t5749;
    let t5752 = t5751 * t109;
    let t5757 = t934 * t574;
    let t5888 = t570 * t352;
    (t5751, t5752, t5757, t5888)
}
