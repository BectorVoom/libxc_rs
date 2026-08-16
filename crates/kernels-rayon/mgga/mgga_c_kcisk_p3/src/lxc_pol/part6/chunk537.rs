//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 537/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk537(t1692: f64, t2399: f64, t2469: f64, t4826: f64, t1907: f64, t2541: f64, t718: f64, t733: f64, t1755: f64, t41: f64, t5320: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7278 = t2399 * t1692;
    let t7283 = t2469 * t4826;
    let t7293 = t2541 * t1907;
    let t7302 = t733 * t718;
    let t7303 = t41 * t1755;
    let t7310 = t739 * t5320;
    (t7278, t7283, t7293, t7302, t7303, t7310)
}
