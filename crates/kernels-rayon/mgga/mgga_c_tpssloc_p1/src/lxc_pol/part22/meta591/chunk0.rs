//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2106/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2106(t116: f64, t212: f64, t2570: f64, t2585: f64, t4255: f64, t2628: f64, t2691: f64, t4184: f64, t812: f64, t1512: f64, t41362: f64, t13176: f64, t2629: f64) -> (f64, f64, f64, f64, f64) {
    let t46853 = t116 * t212;
    let t46855 = t2585 * t2570 * t46853 * t4255;
    let t46856 = 0.14999999999999999999e-1_f64 * t46855;
    let t46874 = t812 * t2628 * t2691 * t4184;
    let t46875 = 119.0_f64 / 2304.0_f64 * t46874;
    let t46876 = t41362 * t1512;
    let t46878 = t13176 * t2629;
    (t46853, t46856, t46875, t46876, t46878)
}
