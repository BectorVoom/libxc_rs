//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 158/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk158<F: Float>(t606: F, t25: F, t353: F, t579: F, t609: F, t45: F, t608: F, t67: F, t227: F, t8: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t612 = pow_3_2::<F>(t606);
    let t615 = t353 * t25 * t579;
    let t617 = F::cast_from(0.379785e1_f64) * t609 + F::cast_from(0.8969e0_f64) * t606 + F::cast_from(0.204775e0_f64) * t612 + F::cast_from(0.24647e0_f64) * t615;
    let t620 = F::cast_from(1.0_f64) + F::cast_from(0.16081824322151104822e2_f64) / t617;
    let t621 = F::ln(t620);
    let t625 = F::cast_from(1.0_f64) + F::cast_from(0.278125e-1_f64) * t606;
    let t630 = F::cast_from(0.51785e1_f64) * t609 + F::cast_from(0.905775e0_f64) * t606 + F::cast_from(0.1100325e0_f64) * t612 + F::cast_from(0.248355e0_f64) * t615;
    let t633 = F::cast_from(1.0_f64) + F::cast_from(0.29608574643216675549e2_f64) / t630;
    let t634 = F::ln(t633);
    let t638 = -F::cast_from(0.62182e-1_f64) * t608 * t621 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t625 * t634;
    let t639 = t67 * t638;
    let t640 = t8 * t227;
    let t641 = pow_1_3::<F>(t640);
    (t615, t617, t620, t621, t625, t630, t633, t634, t638, t639, t640, t641)
}
