//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 504/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk504(t5455: f64, t5456: f64, t5458: f64, t5472: f64, t1439: f64, t453: f64, t1156: f64, t592: f64, t1144: f64, t589: f64, t4396: f64, t521: f64) -> (f64, f64, f64, f64, f64) {
    let t5474 = t5455 + t5456 + t5458 + t5472;
    let t5477 = t1439 * t453;
    let t5480 = t592 * t1156;
    let t5491 = t589 * t1144;
    let t5498 = t4396 * t521;
    (t5474, t5477, t5480, t5491, t5498)
}
