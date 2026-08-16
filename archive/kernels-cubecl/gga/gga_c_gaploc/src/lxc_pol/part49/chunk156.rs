//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 156/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk156<F: Float>(t738: F, t740: F, t270: F, t301: F, t337: F, t359: F, t642: F, t645: F, t648: F, t650: F, t681: F, t703: F, t726: F, t731: F, t735: F) -> F {
    let t741 = t738 * t740;
    let t744 = t337 + t359 + t642 - t645 - t648 + F::cast_from(0.10254034973522965712e-1_f64) * t650 * t301 + F::cast_from(0.76905262301422242837e-2_f64) * t681 * t301 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t703 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t726 - F::cast_from(0.85450291446024714263e-3_f64) * t731 * t735 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t741;
    t744
}
