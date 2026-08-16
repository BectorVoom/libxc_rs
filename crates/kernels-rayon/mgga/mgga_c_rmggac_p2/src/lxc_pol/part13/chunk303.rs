//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 303/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk303(t1602: f64, t305: f64, t1361: f64, t797: f64, t1365: f64, t838: f64, t1369: f64, t326: f64, t1587: f64, t27: f64, t29: f64, t847: f64) -> (f64, f64, f64, f64, f64) {
    let t1603 = t305 * t1602;
    let t1605 = t797 * t1361;
    let t1607 = t838 * t1365;
    let t1609 = t326 * t1369;
    let t1612 = t1587 * t29 * t27;
    let t1614 = 5.0_f64 / 18.0_f64 * t1612 + t847;
    (t1603, t1605, t1607, t1609, t1614)
}
