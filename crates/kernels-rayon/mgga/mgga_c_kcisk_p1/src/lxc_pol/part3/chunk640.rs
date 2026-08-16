//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 640/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk640(t5014: f64, t662: f64, t1310: f64, t657: f64, t718: f64, t733: f64, t1755: f64, t41: f64, t5320: f64, t739: f64, t5330: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7242 = t5014 * t662;
    let t7261 = t1310 * t657;
    let t7302 = t733 * t718;
    let t7303 = t41 * t1755;
    let t7310 = t739 * t5320;
    let t7311 = t79 * t5330;
    (t7242, t7261, t7302, t7303, t7310, t7311)
}
