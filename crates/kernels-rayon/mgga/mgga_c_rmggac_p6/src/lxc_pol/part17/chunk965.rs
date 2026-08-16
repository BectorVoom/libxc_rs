//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 965/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk965(t1971: f64, t236: f64, t6130: f64, t7365: f64, t495: f64, t7231: f64, t8517: f64, t9988: f64, t4601: f64, t9999: f64, t10053: f64, t25918: f64) -> (f64, f64, f64, f64) {
    let t45994 = t7365 * t1971 * t236 * t6130;
    let t45999 = t8517 * t7231 * t236 * t9988 * t495;
    let t46001 = t4601 * t9999;
    let t46003 = t25918 * t10053;
    (t45994, t45999, t46001, t46003)
}
