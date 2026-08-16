//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1178/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1178(t12985: f64, t9577: f64, t41189: f64, t4134: f64, t1489: f64, t41083: f64, t133: f64, t1484: f64, t41214: f64, t6600: f64, t1512: f64, t41362: f64) -> (f64, f64, f64, f64, f64) {
    let t46764 = t9577 * t12985;
    let t46772 = t41189 * t4134;
    let t46790 = t41083 * t1489;
    let t46806 = t41214 * t133 * t6600 * t1484;
    let t46876 = t41362 * t1512;
    (t46764, t46772, t46790, t46806, t46876)
}
