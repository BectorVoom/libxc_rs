//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1285/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1285(t6327: f64, t8009: f64, t18796: f64, t3038: f64, t6317: f64, t8189: f64, t8192: f64, t18790: f64, t8195: f64, t18609: f64, t3074: f64, t6137: f64, t8199: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22528 = 6.0_f64 * t8009 * t6327;
    let t22530 = 6.0_f64 * t18796 * t3038;
    let t22532 = 12.0_f64 * t6317 * t8189;
    let t22534 = 6.0_f64 * t6317 * t8192;
    let t22536 = 0.28947563097646563121e3_f64 * t18790 * t8195;
    let t22538 = 0.48245938496077605201e2_f64 * t18609 * t3074;
    let t22540 = 0.96491876992155210402e2_f64 * t6137 * t8199;
    (t22528, t22530, t22532, t22534, t22536, t22538, t22540)
}
