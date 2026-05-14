//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 965/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk965<F: Float>(t43986: F, t43989: F, t43991: F, t43993: F, t43994: F, t43997: F, t44002: F, t44005: F, t44010: F, t44012: F, t47462: F, t47463: F, t44018: F, t44020: F, t44022: F, t44024: F, t44027: F, t44029: F, t44030: F, t44038: F, t44040: F, t44042: F, t44046: F, t44048: F) -> (F, F) {
    let t47466 = t43986 - t43989 + 0.14896037479937677779e-1 * t43991 + t47462 - t43993 - t43994 - t47463 - 0.11502877786176224903e2 * t43997 + t44002 + t44005 + t44010 + 0.29792074959875355558e-1 * t44012;
    let t47477 = 0.15337170381568299871e2 * t44018 + 0.35750489951850426669e0 * t44020 + 0.35750489951850426669e0 * t44022 + 0.35750489951850426669e0 * t44024 + t44027 + t44029 - 0.46011511144704899612e1 * t44030 - t44038 - t44040 - 0.61348681526273199483e1 * t44042 + t44046 + 0.27606906686822939767e2 * t44048;
    (t47466, t47477)
}
