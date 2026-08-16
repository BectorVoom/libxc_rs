//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1140/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1140<F: Float>(t44018: F, t44020: F, t44022: F, t44024: F, t44027: F, t44029: F, t44030: F, t44038: F, t44040: F, t44042: F, t44046: F, t44048: F) -> F {
    let t47477 = F::cast_from(0.15337170381568299871e2_f64) * t44018 + F::cast_from(0.35750489951850426669e0_f64) * t44020 + F::cast_from(0.35750489951850426669e0_f64) * t44022 + F::cast_from(0.35750489951850426669e0_f64) * t44024 + t44027 + t44029 - F::cast_from(0.46011511144704899612e1_f64) * t44030 - t44038 - t44040 - F::cast_from(0.61348681526273199483e1_f64) * t44042 + t44046 + F::cast_from(0.27606906686822939767e2_f64) * t44048;
    t47477
}
