//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 49/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk49<F: Float>(t104: F, t111: F, t113: F, t120: F, t89: F) -> F {
    let t122 = -F::cast_from(0.59778596625315888114e-2_f64) * t89 + F::new(0.1317375e-2) * t104 - F::new(0.23775e-3) * t111 + F::cast_from(0.64744236347453835951e-5_f64) * t113 - F::cast_from(0.540140625e-6_f64) * t120;
    t122
}
