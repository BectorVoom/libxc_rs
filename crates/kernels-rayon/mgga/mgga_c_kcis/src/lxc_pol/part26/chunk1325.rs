//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1325/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1325(t1394: f64, t27364: f64, t6904: f64, t22271: f64, t5780: f64, t7923: f64, t20975: f64, t27387: f64, t20980: f64, t20985: f64, t21894: f64, t1014: f64, t29340: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t102698 = t1394 * t27364 * t6904;
    let t102701 = t5780 * t7923 * t22271;
    let t102706 = t1394 * t27387 * t20975;
    let t102709 = t1394 * t7923 * t20980;
    let t102712 = t1394 * t7923 * t20985;
    let t102715 = t5780 * t7923 * t21894;
    let t102723 = t1014 * t29340;
    (t102698, t102701, t102706, t102709, t102712, t102715, t102723)
}
