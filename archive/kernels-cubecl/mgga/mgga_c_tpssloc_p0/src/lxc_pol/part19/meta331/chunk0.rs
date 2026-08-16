//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1181/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1181<F: Float>(t40419: F, t535: F, t9538: F, t12231: F, t3726: F, t12199: F, t12208: F, t118: F, t12012: F, t3739: F, t794: F, t12217: F, t40021: F) -> (F, F, F, F, F) {
    let t40422 = F::cast_from(0.26851851851851851851e-2_f64) * t40419 * t535 * t9538;
    let t40423 = t3726 * t12231;
    let t40425 = t12199 * t12208;
    let t40429 = t3739 * t118 * t794 * t12012;
    let t40431 = t40021 * t12217;
    (t40422, t40423, t40425, t40429, t40431)
}
