//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1140/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1140(t44018: f64, t44020: f64, t44022: f64, t44024: f64, t44027: f64, t44029: f64, t44030: f64, t44038: f64, t44040: f64, t44042: f64, t44046: f64, t44048: f64) -> f64 {
    let t47477 = 0.15337170381568299871e2_f64 * t44018 + 0.35750489951850426669e0_f64 * t44020 + 0.35750489951850426669e0_f64 * t44022 + 0.35750489951850426669e0_f64 * t44024 + t44027 + t44029 - 0.46011511144704899612e1_f64 * t44030 - t44038 - t44040 - 0.61348681526273199483e1_f64 * t44042 + t44046 + 0.27606906686822939767e2_f64 * t44048;
    t47477
}
