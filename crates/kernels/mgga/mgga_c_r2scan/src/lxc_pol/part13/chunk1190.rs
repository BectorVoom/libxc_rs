//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1190/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1190<F: Float>(t3578: F, t494: F, t97: F, t11004: F, t113: F, t11505: F, t11012: F, t1543: F, t2867: F, t10610: F, t3263: F, t2259: F, t3582: F) -> (F, F, F, F, F, F) {
    let t40276 = t97 * t3578 * t494;
    let t40278 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t40276 * t11004;
    let t40282 = t97 * t11505 * t113;
    let t40284 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t40282 * t11012;
    let t40285 = t2867 * t1543;
    let t40288 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t10610 * t3263 * t40285;
    let t40289 = t3582 * t2259;
    (t40276, t40278, t40282, t40284, t40288, t40289)
}
