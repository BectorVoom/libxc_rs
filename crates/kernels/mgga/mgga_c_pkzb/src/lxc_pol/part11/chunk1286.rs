//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1286/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1286<F: Float>(t27937: F, t3038: F, t27494: F, t3074: F, t8009: F, t9856: F, t8219: F, t9860: F, t22823: F, t9853: F, t9864: F, t22727: F, t9868: F) -> (F, F, F, F, F, F, F) {
    let t31327 = F::new(6.0) * t27937 * t3038;
    let t31329 = F::cast_from(0.48245938496077605201e2_f64) * t27494 * t3074;
    let t31331 = F::new(6.0) * t8009 * t9856;
    let t31333 = F::cast_from(0.48245938496077605201e2_f64) * t8219 * t9860;
    let t31335 = F::cast_from(0.2894756309764656312e3_f64) * t22823 * t9853;
    let t31337 = F::cast_from(0.96491876992155210402e2_f64) * t8219 * t9864;
    let t31339 = F::cast_from(0.1551780387578202009e4_f64) * t22727 * t9868;
    (t31327, t31329, t31331, t31333, t31335, t31337, t31339)
}
