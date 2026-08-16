//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1154/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1154(t14650: f64, t3202: f64, t14627: f64, t1697: f64, t2835: f64, t10477: f64, t14624: f64, t14631: f64, t14635: f64, t14638: f64, t14642: f64, t14644: f64, t14647: f64, t1710: f64, t2812: f64, t9565: f64) -> (f64, f64) {
    let t14651 = t3202 * t14650;
    let t14652 = t14627 * t14651;
    let t14654 = t1697 * t2835;
    let t14659 = -0.2653111111111111111e-1_f64 * t14624 + 0.66327777777777777776e-2_f64 * t14631 - 0.22109259259259259258e-2_f64 * t14635 - 0.22109259259259259258e-2_f64 * t10477 - 0.33163888888888888888e-2_f64 * t14638 + 0.99491666666666666664e-2_f64 * t14642 + 0.22109259259259259258e-2_f64 * t14644 - 0.58958024691358024689e-2_f64 * t14647 + 0.11054629629629629629e-2_f64 * t14652 + 0.890445125e-2_f64 * t14654 * t2812 - 0.66725e-1_f64 * t9565 * t1710;
    (t14652, t14659)
}
