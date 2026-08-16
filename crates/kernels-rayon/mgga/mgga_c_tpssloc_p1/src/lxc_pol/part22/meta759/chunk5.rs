//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2554/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2554(t43816: f64, t51039: f64, t51051: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t64074: f64, t64076: f64, t64087: f64, t64089: f64) -> f64 {
    let t71649 = -0.31003950617283950618e0_f64 * t43816 + 0.54771111111111111112e0_f64 * t51039 - 0.91285185185185185187e-1_f64 * t51051 + 0.79724444444444444444e0_f64 * t63361 + 0.39862222222222222222e0_f64 * t63382 + 0.11958666666666666667e1_f64 * t63384 - 0.11958666666666666667e1_f64 * t63398 - 0.17938e1_f64 * t63400 + 0.10954222222222222222e0_f64 * t64074 + 0.32862666666666666666e0_f64 * t64076 - 0.65725333333333333332e0_f64 * t64087 - 0.98587999999999999998e0_f64 * t64089;
    t71649
}
