//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2683/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2683<F: Float>(t39510: F, t39512: F, t39514: F, t39522: F, t39530: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F) -> (F, F, F, F, F, F) {
    let t56365 = F::cast_from(0.10843581300301739842e-1_f64) * t39510;
    let t56366 = F::cast_from(0.96319466275353142156e0_f64) * t39512;
    let t56367 = F::cast_from(0.43374325201206959367e-1_f64) * t39514;
    let t56368 = F::cast_from(0.65061487801810439052e-1_f64) * t39522;
    let t56369 = F::cast_from(0.17315859105681463759e2_f64) * t39530;
    let t56370 = t39499 + t39502 - t39505 - t39508 + t56365 + t56366 - t56367 + t39518 - t39521 - t56368 - t39529 - t56369;
    (t56365, t56366, t56367, t56368, t56369, t56370)
}
