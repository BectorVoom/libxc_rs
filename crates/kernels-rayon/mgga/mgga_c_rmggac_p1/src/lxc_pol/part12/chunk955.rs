//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 955/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk955(t2310: f64, t35277: f64, t1525: f64, t236: f64, t321: f64, t3352: f64, t7230: f64, t615: f64, t833: f64, t34847: f64, t8836: f64, t1971: f64, t333: f64, t511: f64) -> (f64, f64, f64, f64, f64) {
    let t40367 = t35277 * t2310;
    let t40372 = t7230 * t3352 * t236 * t1525 * t321;
    let t40377 = t7230 * t3352 * t236 * t615 * t833;
    let t40379 = t34847 * t8836;
    let t40384 = t7230 * t1971 * t511 * t1525 * t333;
    (t40367, t40372, t40377, t40379, t40384)
}
