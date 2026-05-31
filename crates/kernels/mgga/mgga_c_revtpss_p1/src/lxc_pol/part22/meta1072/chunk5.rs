//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3847/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3847<F: Float>(t1410: F, t1414: F, t22079: F, t4004: F, t46649: F, t46652: F, t48486: F, t48488: F, t48494: F, t48498: F, t5671: F, t5673: F, t73578: F, t73975: F, t73985: F, t73994: F, t73998: F, t828: F) -> F {
    let t74004 = F::cast_from(0.57165357490759649296e-3_f64) * t73975 + F::cast_from(0.12862205435420921092e-2_f64) * t5671 * t5673 * t22079 * t4004 + F::cast_from(0.10164000561857065645e-4_f64) * t46649 + F::cast_from(455.0_f64) / F::cast_from(324.0_f64) * t46652 - F::cast_from(0.85748036236139473945e-4_f64) * t73985 + F::cast_from(0.45351183609335988443e-1_f64) * t48486 - F::cast_from(0.2168320119862840671e-2_f64) * t48488 + F::cast_from(0.10164000561857065645e-3_f64) * t48494 - F::cast_from(0.50820002809285328225e-3_f64) * t48498 + F::cast_from(0.17149607247227894789e-2_f64) * t73994 - F::cast_from(0.57165357490759649296e-3_f64) * t73998 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t1414 * t828 * t73578;
    t74004
}
