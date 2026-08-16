//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1019/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1019<F: Float>(t31: F, t625: F, t31864: F, t607: F, t8308: F, t63: F, t645: F, t31857: F, t32344: F, t31868: F, t240: F, t8307: F, t8513: F, t8663: F) -> (F, F, F, F, F) {
    let t117496 = t625 * t31;
    let t117499 = t31864 * t8308 * t117496 * t607;
    let t117503 = t63 * t645;
    let t117516 = t31857 * t32344;
    let t117518 = t31868 * t32344;
    let t117527 = F::cast_from(55.0_f64) / F::cast_from(81.0_f64) * t8663 * t8513 * t8307 * t240;
    (t117499, t117503, t117516, t117518, t117527)
}
