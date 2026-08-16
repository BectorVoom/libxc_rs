//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 900/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk900<F: Float>(t11203: F, t1114: F, t2403: F, t241: F, t3439: F, t407: F, t11135: F, t410: F, t417: F, t1097: F, t3311: F, t409: F) -> (F, F, F, F, F, F, F) {
    let t11204 = F::cast_from(0.36514074074074074075e0_f64) * t11203;
    let t11211 = t2403 * t1114;
    let t11219 = t241 * t3439;
    let t11243 = F::cast_from(1.0_f64)/pow_3_2::<F>(t407);
    let t11247 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t11135;
    let t11265 = F::cast_from(1.0_f64) / t410 / t417 / F::cast_from(4.0_f64);
    let t11274 = F::cast_from(1.0_f64) / t3311 / t1097;
    let t11275 = t409 * t11274;
    (t11204, t11211, t11219, t11243, t11247, t11265, t11275)
}
