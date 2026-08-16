//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1182/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1182(t10497: f64, t1778: f64, t3329: f64, t5034: f64, t1239: f64, t15469: f64, t1262: f64, t5336: f64, t110: f64, t287: f64) -> (f64, f64, f64, f64, f64) {
    let t46026 = t1778 * t10497;
    let t46041 = t5034 * t3329;
    let t46577 = t15469 * t1239;
    let t46849 = t5336 * t1262;
    let t46978 = t110 * t287;
    (t46026, t46041, t46577, t46849, t46978)
}
