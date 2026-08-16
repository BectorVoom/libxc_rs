//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 522/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk522<F: Float>(t3068: F, t3575: F, t1244: F, t1230: F, t820: F, t1089: F, t415: F, t1236: F, t225: F, t1239: F, t496: F, t68: F) -> (F, F, F, F, F, F) {
    let t3576 = t3575 * t3068;
    let t3577 = t1244 * t3576;
    let t3578 = t820 * t1230;
    let t3584 = F::cast_from(1.0_f64) / t415 / t1089;
    let t3593 = t1236 * t225;
    let t3597 = F::cast_from(1.0_f64) / t1239 / t496;
    let t3598 = t68 * t3597;
    (t3577, t3578, t3584, t3593, t3597, t3598)
}
