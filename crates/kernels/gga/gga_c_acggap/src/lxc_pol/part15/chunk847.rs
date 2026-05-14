//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 847/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk847<F: Float>(t30232: F, t30238: F, t30242: F, t30246: F, t30339: F, t30396: F, t30405: F, t30421: F, t30428: F, t30568: F, t30576: F, t30590: F, t30595: F, t30657: F, t30663: F, t30669: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32398 = 0.83861579438944405516e-2 * t30232;
    let t32401 = 0.21437009059034868486e-3 * t30238;
    let t32403 = 0.42874018118069736972e-3 * t30242;
    let t32404 = 0.68026775414003982662e-1 * t30246;
    let t32435 = 0.12862205435420921092e-2 * t30339;
    let t32456 = 5.0 / 64.0 * t30396;
    let t32458 = 0.25724410870841842183e-2 * t30405;
    let t32461 = 0.37737710747524982482e-2 * t30421;
    let t32462 = 5.0 / 288.0 * t30428;
    let t32507 = 0.11321313224257494745e-1 * t30568;
    let t32509 = 0.85748036236139473944e-3 * t30576;
    let t32515 = 0.57165357490759649297e-2 * t30590;
    let t32517 = 0.74085763888888888887e0 * t30595;
    let t32540 = 0.64311027177104605458e-3 * t30657;
    let t32543 = 0.38586616306262763276e-2 * t30663;
    let t32544 = 0.72028350438357158115e-1 * t30669;
    (t32398, t32401, t32403, t32404, t32435, t32456, t32458, t32461, t32462, t32507, t32509, t32515, t32517, t32540, t32543, t32544)
}
