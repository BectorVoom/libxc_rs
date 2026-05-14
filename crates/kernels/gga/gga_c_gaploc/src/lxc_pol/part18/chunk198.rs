//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 198/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk198<F: Float>(t738: F, t740: F, t270: F, t301: F, t337: F, t359: F, t642: F, t645: F, t648: F, t650: F, t681: F, t703: F, t726: F, t731: F, t735: F, t330: F) -> (F, F, F) {
    let t741 = t738 * t740;
    let t744 = t337 + t359 + t642 - t645 - t648 + 0.10254034973522965712e-1 * t650 * t301 + 0.76905262301422242837e-2 * t681 * t301 - 0.76905262301422242837e-2 * t270 * t703 + 0.76905262301422242837e-2 * t270 * t726 - 0.85450291446024714263e-3 * t731 * t735 - 0.76905262301422242837e-2 * t270 * t741;
    let t746 = t330 * t330;
    (t741, t744, t746)
}
