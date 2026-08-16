//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 742/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk742(t7754: f64, t930: f64, t2010: f64, t7756: f64, t118: f64, t2001: f64, t353: f64, t498: f64, t1212: f64, t128: f64, t1986: f64, t209: f64) -> (f64, f64, f64) {
    let t35000 = t7754 * t930;
    let t35002 = t2010 * t35000 * t7756;
    let t35018 = t2001 * t118 * t353 * t498;
    let t35024 = t1986 * t118 * t128 * t1212 * t209;
    (t35002, t35018, t35024)
}
