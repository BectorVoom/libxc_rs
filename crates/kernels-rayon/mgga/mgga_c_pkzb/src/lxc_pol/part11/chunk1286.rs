//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1286/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1286(t27937: f64, t3038: f64, t27494: f64, t3074: f64, t8009: f64, t9856: f64, t8219: f64, t9860: f64, t22823: f64, t9853: f64, t9864: f64, t22727: f64, t9868: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31327 = 6.0_f64 * t27937 * t3038;
    let t31329 = 0.48245938496077605201e2_f64 * t27494 * t3074;
    let t31331 = 6.0_f64 * t8009 * t9856;
    let t31333 = 0.48245938496077605201e2_f64 * t8219 * t9860;
    let t31335 = 0.2894756309764656312e3_f64 * t22823 * t9853;
    let t31337 = 0.96491876992155210402e2_f64 * t8219 * t9864;
    let t31339 = 0.1551780387578202009e4_f64 * t22727 * t9868;
    (t31327, t31329, t31331, t31333, t31335, t31337, t31339)
}
