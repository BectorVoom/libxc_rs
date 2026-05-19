//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 937/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk937<F: Float>(t30428: F, t30568: F, t30576: F, t30590: F, t30595: F, t30657: F, t30663: F, t30669: F, t30671: F, t30714: F, t30728: F, t30882: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32462 = F::new(5.0) / F::new(288.0) * t30428;
    let t32507 = F::cast_from(0.11321313224257494745e-1_f64) * t30568;
    let t32509 = F::cast_from(0.85748036236139473944e-3_f64) * t30576;
    let t32515 = F::cast_from(0.57165357490759649297e-2_f64) * t30590;
    let t32517 = F::cast_from(0.74085763888888888887e0_f64) * t30595;
    let t32540 = F::cast_from(0.64311027177104605458e-3_f64) * t30657;
    let t32543 = F::cast_from(0.38586616306262763276e-2_f64) * t30663;
    let t32544 = F::cast_from(0.72028350438357158115e-1_f64) * t30669;
    let t32545 = F::cast_from(0.10566559009306995095e0_f64) * t30671;
    let t32557 = F::new(0.2546875e-1) * t30714;
    let t32561 = F::cast_from(0.11321313224257494745e-1_f64) * t30728;
    let t32619 = F::cast_from(0.21881628506185221314e-1_f64) * t30882;
    (t32462, t32507, t32509, t32515, t32517, t32540, t32543, t32544, t32545, t32557, t32561, t32619)
}
