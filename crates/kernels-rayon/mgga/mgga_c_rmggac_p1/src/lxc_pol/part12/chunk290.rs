//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 290/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk290(t1451: f64, t201: f64, t228: f64, t457: f64, t597: f64, t461: f64, t615: f64, t495: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1452 = t1451 * t201;
    let t1453 = t1452 * t228;
    let t1454 = t597 * t457;
    let t1455 = t201 * t228;
    let t1459 = t461 * t615;
    let t1462 = t615 * t495;
    (t1452, t1453, t1454, t1455, t1459, t1462)
}
