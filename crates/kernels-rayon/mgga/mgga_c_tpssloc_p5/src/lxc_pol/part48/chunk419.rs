//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 419/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk419(t2836: f64, t913: f64, t893: f64, t891: f64, t275: f64, t290: f64, t2793: f64, t2764: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64) -> (f64, f64, f64) {
    let t2837 = t2836 * t913;
    let t2839 = 1.0_f64 * t893 * t2837;
    let t2840 = t891 * t891;
    let t2841 = 1.0_f64 / t2840;
    let t2842 = t275 * t2841;
    let t2843 = t290 * t290;
    let t2844 = 1.0_f64 / t2843;
    let t2845 = t2793 * t2844;
    let t2847 = 0.16081979498692535067e2_f64 * t2842 * t2845;
    let t2848 = 0.22831111111111111111e-1_f64 * t2764;
    let t2853 = t2848 + 0.11415555555555555555e-1_f64 * t2766 - 0.11415555555555555555e-1_f64 * t2773 + 0.34246666666666666666e-1_f64 * t2778 - 0.17123333333333333333e-1_f64 * t2782;
    (t2839, t2847, t2853)
}
