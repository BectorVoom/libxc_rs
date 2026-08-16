//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2382/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2382<F: Float>(t136: F, t68554: F, t908: F, t43317: F, t48140: F, t68513: F, t49200: F, t59657: F, t60163: F, t60168: F, t60173: F, t68536: F, t68541: F, t68545: F, t68549: F, t68552: F) -> (F, F, F) {
    let t68556 = t136 * t908 * t68554;
    let t68563 = t48140 * t43317 * t68513;
    let t68565 = F::cast_from(0.16557e0_f64) * t68536 - F::cast_from(0.27595e-1_f64) * t68541 + F::cast_from(0.198684e1_f64) * t68545 - F::cast_from(0.149013e1_f64) * t68549 - F::cast_from(0.99342e0_f64) * t68552 + F::cast_from(0.49671e0_f64) * t68556 + F::cast_from(0.16557e0_f64) * t60163 + F::cast_from(0.5519e0_f64) * t60168 - F::cast_from(0.27595e0_f64) * t60173 - F::cast_from(0.26837777777777777777e0_f64) * t59657 - F::cast_from(0.11038e0_f64) * t68563 + t49200;
    (t68556, t68563, t68565)
}
