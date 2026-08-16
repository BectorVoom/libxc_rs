//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2546/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2546(t43816: f64, t51040: f64, t51051: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t64074: f64, t64076: f64, t64087: f64, t64089: f64) -> f64 {
    let t71527 = -0.31310740740740740741e0_f64 * t43816 + t51040 - 0.91983333333333333333e-1_f64 * t51051 + 0.80513333333333333334e0_f64 * t63361 + 0.40256666666666666666e0_f64 * t63382 + 0.12077e1_f64 * t63384 - 0.12077e1_f64 * t63398 - 0.181155e1_f64 * t63400 + 0.11038e0_f64 * t64074 + 0.33114e0_f64 * t64076 - 0.66228e0_f64 * t64087 - 0.99342e0_f64 * t64089;
    t71527
}
