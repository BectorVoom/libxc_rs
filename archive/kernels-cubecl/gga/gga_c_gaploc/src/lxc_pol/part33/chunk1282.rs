//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1282/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1282<F: Float>(t8556: F, t9823: F, t7336: F, t8775: F, t11116: F, t22274: F, t11069: F, t5662: F, t11016: F, t8478: F, t8638: F, t29052: F, t3025: F) -> (F, F, F, F, F, F, F) {
    let t33846 = F::cast_from(0.47667319935800568892e0_f64) * t9823 * t8556;
    let t33848 = F::cast_from(0.2780593662921699852e0_f64) * t8775 * t7336;
    let t33851 = F::cast_from(0.1853729108614466568e0_f64) * t22274 * t11116;
    let t33853 = F::cast_from(0.1022478025437886658e1_f64) * t5662 * t11069;
    let t33857 = F::cast_from(0.14300195980740170668e1_f64) * t8478 * t11016;
    let t33859 = F::cast_from(0.14300195980740170668e1_f64) * t8638 * t11016;
    let t33861 = F::cast_from(0.14300195980740170668e1_f64) * t3025 * t29052;
    (t33846, t33848, t33851, t33853, t33857, t33859, t33861)
}
