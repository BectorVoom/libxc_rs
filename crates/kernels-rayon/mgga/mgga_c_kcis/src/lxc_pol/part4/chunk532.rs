//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 532/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk532(t169: f64, t171: f64, t2629: f64, t2630: f64, t2635: f64, t176: f64, t833: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t2639 = piecewise3(t170, 0.0_f64, 4.0_f64 / 9.0_f64 * t2629 * t2630 + 4.0_f64 / 3.0_f64 * t171 * t2635);
    let t2640 = t176 * t176;
    let t2641 = 1.0_f64 / t2640;
    let t2642 = t833 * t833;
    (t2639, t2640, t2641, t2642)
}
