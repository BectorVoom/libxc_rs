//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1286/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1286(t6137: f64, t8202: f64, t18612: f64, t8206: f64, t2197: f64, t8004: f64, t851: f64, t2234: f64, t3070: f64, t2198: f64, t6142: f64, t8198: f64) -> (f64, f64, f64, f64, f64) {
    let t22542 = 0.48245938496077605201e2_f64 * t6137 * t8202;
    let t22544 = 0.1551780387578202009e4_f64 * t18612 * t8206;
    let t22547 = 6.0_f64 * t2197 * t8004 * t851;
    let t22550 = 6.0_f64 * t2197 * t3070 * t2234;
    let t22553 = 0.28947563097646563121e3_f64 * t6142 * t8198 * t2198;
    (t22542, t22544, t22547, t22550, t22553)
}
