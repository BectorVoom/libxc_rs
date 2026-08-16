//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 207/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk207<F: Float>(t651: F, t653: F, t657: F, t659: F, t31: F) -> (F, F) {
    let t661 = -F::cast_from(0.632975e0_f64) * t651 - F::cast_from(0.29896666666666666667e0_f64) * t653 - F::cast_from(0.1023875e0_f64) * t657 - F::cast_from(0.82156666666666666667e-1_f64) * t659;
    let t662 = F::cast_from(1.0_f64) / t31;
    (t661, t662)
}
