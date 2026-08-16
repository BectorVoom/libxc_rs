//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1143/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1143<F: Float>(t40419: F, t535: F, t9538: F, t241: F, t6597: F, t248: F, t555: F, t557: F, t40041: F, t562: F, t12019: F, t566: F) -> (F, F, F, F, F) {
    let t40422 = F::cast_from(0.26851851851851851851e-2_f64) * t40419 * t535 * t9538;
    let t40445 = t6597 * t241;
    let t40449 = F::cast_from(13685.0_f64) / F::cast_from(31104.0_f64) * t555 * t40445 * t557 * t248;
    let t40541 = t40041 * t562;
    let t40590 = F::cast_from(1.0_f64) / t12019 / t566;
    (t40422, t40445, t40449, t40541, t40590)
}
