//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 599/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk599<F: Float>(t25653: F, t73: F, t3056: F, t373: F, t35: F, t5537: F, t5546: F, t929: F, t11121: F, t1603: F, t1669: F, t22574: F, t22613: F, t22619: F, t22634: F, t22644: F, t22696: F, t22736: F, t22738: F, t22834: F, t25626: F, t25631: F, t25637: F, t25640: F, t25644: F, t25649: F, t3019: F, t5538: F, t5540: F, t6427: F, t6428: F, t6431: F) -> (F, F, F) {
    let t25654 = t73 * t25653;
    let t25657 = t373 * t3056;
    let t25658 = t25657 * t35;
    let t25663 = t5537 * t5546 * t929;
    let t25669 = F::cast_from(0.13519760450715832853e-3_f64) * t3019 * t25626 - F::cast_from(0.23254900946437792e-1_f64) * t22834 * t6428 - F::cast_from(0.23254900946437792e-1_f64) * t1603 * t25631 + F::cast_from(0.74233839446572641111e-4_f64) * t22574 - F::new(2.0) * t22696 * t6431 - F::new(2.0) * t1669 * t25637 - F::new(2.0) * t1669 * t25640 + F::new(4.0) * t1669 * t25644 + F::cast_from(0.12768721675925925926e-1_f64) * t22634 - F::cast_from(0.15137014751914110597e-3_f64) * t22644 + F::cast_from(0.44540303667943584666e-3_f64) * t22613 * t73 * t25649 - F::cast_from(0.44540303667943584666e-3_f64) * t22619 * t25654 + F::cast_from(0.25845121844514357744e-4_f64) * t5538 * t5540 * t25658 - F::cast_from(0.60102574844279699039e-6_f64) * t11121 * t25663 + F::cast_from(0.61277550024922479209e-6_f64) * t22736 * t22738 * t6427;
    (t25654, t25658, t25669)
}
