//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1048/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1048(t26193: f64, t7691: f64, t6888: f64, t22933: f64, t6439: f64, t6889: f64, t1985: f64, t25: f64, t5527: f64, t1484: f64, t1530: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28213 = t26193 * t7691;
    let t28214 = t6888 * t28213;
    let t28232 = t22933 * t6439;
    let t28233 = t6889 * t28232;
    let t28234 = t1985 * t28233;
    let t28241 = t25 * t5527;
    let t28248 = t1484 * t1530;
    (t28213, t28214, t28232, t28233, t28234, t28241, t28248)
}
