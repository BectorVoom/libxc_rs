//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1082/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1082<F: Float>(t11779: F, t21758: F, t248: F, t1230: F, t21776: F, t21769: F, t1156: F, t21906: F, t3400: F, t1164: F, t4869: F, t6106: F) -> (F, F, F, F, F, F) {
    let t22208 = t248 * t11779 * t21758;
    let t22214 = t248 * t1230 * t21776;
    let t22218 = t248 * t1230 * t21769;
    let t22222 = t3400 * t21906 * t1156;
    let t22224 = F::cast_from(0.35089341735807877242e1_f64) * t1164 * t22222;
    let t22226 = F::cast_from(0.51947577317044391276e2_f64) * t4869 * t6106;
    (t22208, t22214, t22218, t22222, t22224, t22226)
}
