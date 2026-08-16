//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2013/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2013(t22674: f64, t22686: f64, t80681: f64, t22663: f64, t6883: f64, t225: f64, t22624: f64, t22622: f64, t214: f64, t3879: f64, t22675: f64, t22724: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t80683 = t80681 * t22674 * t22686;
    let t80689 = t6883 * t22663;
    let t80699 = t22624 * t225;
    let t80704 = t22622 * t225;
    let t80707 = t214 * t3879;
    let t80711 = t22724 * t22675;
    (t80683, t80689, t80699, t80704, t80707, t80711)
}
