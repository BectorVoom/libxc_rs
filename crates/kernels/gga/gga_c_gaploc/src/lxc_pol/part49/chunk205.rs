//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 205/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk205<F: Float>(t492: F, t993: F, t105: F, t886: F, t989: F, t189: F, t986: F) -> (F, F, F) {
    let t994 = t492 * t993;
    let t997 = F::new(0.28455006635676149599e-1) * t105 * t989 + F::new(0.23712505529730124666e-2) * t886 - F::new(0.28455006635676149599e-1) * t105 * t994;
    let t999 = t189 * t986;
    (t994, t997, t999)
}
