//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2264/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2264(t13326: f64, t9638: f64, t2628: f64, t2691: f64, t4184: f64, t812: f64, t1512: f64, t41362: f64, t13176: f64, t2629: f64, t4166: f64, t9666: f64) -> (f64, f64, f64, f64, f64) {
    let t46870 = t9638 * t13326;
    let t46874 = t812 * t2628 * t2691 * t4184;
    let t46875 = 119.0_f64 / 2304.0_f64 * t46874;
    let t46876 = t41362 * t1512;
    let t46878 = t13176 * t2629;
    let t46881 = t4166 * t9666;
    (t46870, t46875, t46876, t46878, t46881)
}
