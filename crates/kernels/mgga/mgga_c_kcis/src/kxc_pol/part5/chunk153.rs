//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 153/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk153<F: Float>(t250: F, t252: F, t461: F, t453: F, t456: F, t459: F) -> (F, F, F, F) {
    let t463 = t250 * t252 * t461;
    let t465 = F::cast_from(0.379785e1_f64) * t456 + F::cast_from(0.8969e0_f64) * t453 + F::cast_from(0.204775e0_f64) * t459 + F::cast_from(0.123235e0_f64) * t463;
    let t468 = F::cast_from(1.0_f64) + F::cast_from(0.16081824322151104822e2_f64) / t465;
    let t469 = F::ln(t468);
    (t463, t465, t468, t469)
}
