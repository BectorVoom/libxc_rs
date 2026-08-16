//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1336/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1336(t22259: f64, t97793: f64, t4122: f64, t6012: f64, t97800: f64, t5916: f64, t97767: f64, t5913: f64, t3734: f64, t7305: f64, t6029: f64, t97804: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102889 = t97793 * t22259;
    let t102892 = t4122 * t97800 * t6012;
    let t102894 = t97767 * t5916;
    let t102896 = t97767 * t5913;
    let t102898 = t3734 * t7305;
    let t102900 = t97804 * t6029;
    (t102889, t102892, t102894, t102896, t102898, t102900)
}
