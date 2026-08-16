//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1032/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1032(t1818: f64, t1970: f64, t209: f64, t236: f64, t476: f64, t9210: f64, t10082: f64, t495: f64, t7230: f64, t7248: f64, t1916: f64, t2144: f64) -> (f64, f64, f64) {
    let t46969 = t1970 * t9210 * t236 * t1818 * t476 * t209;
    let t46974 = t7230 * t7248 * t236 * t10082 * t495;
    let t46976 = t1916 * t2144;
    (t46969, t46974, t46976)
}
