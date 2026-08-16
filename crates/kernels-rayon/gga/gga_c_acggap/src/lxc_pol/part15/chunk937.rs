//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 937/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk937(t30428: f64, t30568: f64, t30576: f64, t30590: f64, t30595: f64, t30657: f64, t30663: f64, t30669: f64, t30671: f64, t30714: f64, t30728: f64, t30882: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32462 = 5.0_f64 / 288.0_f64 * t30428;
    let t32507 = 0.11321313224257494745e-1_f64 * t30568;
    let t32509 = 0.85748036236139473944e-3_f64 * t30576;
    let t32515 = 0.57165357490759649297e-2_f64 * t30590;
    let t32517 = 0.74085763888888888887e0_f64 * t30595;
    let t32540 = 0.64311027177104605458e-3_f64 * t30657;
    let t32543 = 0.38586616306262763276e-2_f64 * t30663;
    let t32544 = 0.72028350438357158115e-1_f64 * t30669;
    let t32545 = 0.10566559009306995095e0_f64 * t30671;
    let t32557 = 0.2546875e-1_f64 * t30714;
    let t32561 = 0.11321313224257494745e-1_f64 * t30728;
    let t32619 = 0.21881628506185221314e-1_f64 * t30882;
    (t32462, t32507, t32509, t32515, t32517, t32540, t32543, t32544, t32545, t32557, t32561, t32619)
}
