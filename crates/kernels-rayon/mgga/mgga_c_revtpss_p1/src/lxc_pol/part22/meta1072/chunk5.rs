//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3847/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3847(t1410: f64, t1414: f64, t22079: f64, t4004: f64, t46649: f64, t46652: f64, t48486: f64, t48488: f64, t48494: f64, t48498: f64, t5671: f64, t5673: f64, t73578: f64, t73975: f64, t73985: f64, t73994: f64, t73998: f64, t828: f64) -> f64 {
    let t74004 = 0.57165357490759649296e-3_f64 * t73975 + 0.12862205435420921092e-2_f64 * t5671 * t5673 * t22079 * t4004 + 0.10164000561857065645e-4_f64 * t46649 + 455.0_f64 / 324.0_f64 * t46652 - 0.85748036236139473945e-4_f64 * t73985 + 0.45351183609335988443e-1_f64 * t48486 - 0.2168320119862840671e-2_f64 * t48488 + 0.10164000561857065645e-3_f64 * t48494 - 0.50820002809285328225e-3_f64 * t48498 + 0.17149607247227894789e-2_f64 * t73994 - 0.57165357490759649296e-3_f64 * t73998 - 0.85748036236139473944e-3_f64 * t1410 * t1414 * t828 * t73578;
    t74004
}
