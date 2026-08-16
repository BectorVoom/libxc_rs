//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 224/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk224(t1551: f64, t793: f64, t1554: f64, t797: f64, t338: f64, t551: f64, t352: f64, t305: f64, t128: f64, t1587: f64, t1361: f64, t1365: f64, t838: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1594 = t793 * t1551;
    let t1596 = t797 * t1554;
    let t1598 = t338 * t551;
    let t1599 = t1598 * t352;
    let t1600 = t305 * t1599;
    let t1602 = t128 * t1587;
    let t1603 = t305 * t1602;
    let t1605 = t797 * t1361;
    let t1607 = t838 * t1365;
    (t1594, t1596, t1600, t1602, t1603, t1605, t1607)
}
