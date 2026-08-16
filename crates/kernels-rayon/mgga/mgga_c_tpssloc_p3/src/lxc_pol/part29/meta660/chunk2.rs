//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2193/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2193(t22635: f64, t26331: f64, t26332: f64, t3719: f64, t1834: f64, t213: f64, t225: f64, t22633: f64, t22637: f64, t26333: f64, t80650: f64, t16470: f64, t26224: f64, t26225: f64, t80689: f64, t90539: f64, t90542: f64, t90547: f64, t90550: f64, t90551: f64, t90556: f64) -> f64 {
    let t90560 = t26331 * t22635 * t26332 * t3719;
    let t90566 = t213 * t1834 * t225;
    let t90568 = t22633 * t90566 * t22637;
    let t90571 = t26331 * t80650 * t26333;
    let t90573 = 0.16449340668482264365e-1_f64 * t90539 + t90542 + 0.19190897446562641759e-1_f64 * t80689 + t90547 - t90550 - 0.52089578783527170489e-1_f64 * t90551 + 0.9869604401089358619e-1_f64 * t90556 + 0.49348022005446793095e-1_f64 * t90560 - 6.0_f64 * t26224 * t26225 * t16470 + 0.3289868133696452873e-1_f64 * t90568 + 0.9869604401089358619e-1_f64 * t90571;
    t90573
}
