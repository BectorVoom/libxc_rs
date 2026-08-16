//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 837/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk837(t1401: f64, t1458: f64, t2039: f64, t3941: f64, t5371: f64, t577: f64, t7230: f64, t7801: f64, t7945: f64, t7956: f64, t1409: f64, t1419: f64, t56: f64, t6503: f64, t7251: f64) -> (f64, f64) {
    let t7961 = 0.45e1_f64 * t7945 * t577 + 0.135e2_f64 * t7230 * t1458 + 0.135e2_f64 * t5371 * t2039 + 27.0_f64 * t3941 * t7956 + 0.135e2_f64 * t1401 * t7801;
    let t7973 = -8.0_f64 / 3.0_f64 * t1419 * t56 - 5.0_f64 / 6.0_f64 * t7251 * t1409 + t6503;
    (t7961, t7973)
}
