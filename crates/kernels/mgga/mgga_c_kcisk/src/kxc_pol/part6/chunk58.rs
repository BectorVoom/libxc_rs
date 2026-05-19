//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 58/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk58<F: Float>(t143: F, t158: F, t165: F, t167: F, t173: F) -> F {
    let t175 = -F::cast_from(0.59778596625315888114e-2_f64) * t143 + F::new(0.1317375e-2) * t158 - F::new(0.23775e-3) * t165 + F::cast_from(0.64744236347453835951e-5_f64) * t167 - F::cast_from(0.540140625e-6_f64) * t173;
    t175
}
