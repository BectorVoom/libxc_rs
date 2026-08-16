//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1123/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1123(t209: f64, t736: f64, t2887: f64, t291: f64, t10497: f64, t1778: f64, t3329: f64, t5034: f64, t110: f64, t287: f64) -> (f64, f64, f64, f64, f64) {
    let t44682 = t209 * t736;
    let t44756 = t2887 * t291;
    let t46026 = t1778 * t10497;
    let t46041 = t5034 * t3329;
    let t46978 = t110 * t287;
    (t44682, t44756, t46026, t46041, t46978)
}
