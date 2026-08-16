//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 609/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk609(t290: f64, t2793: f64, t2842: f64, t2764: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t919: f64, t923: f64, t307: f64, t922: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2843 = t290 * t290;
    let t2844 = 1.0_f64 / t2843;
    let t2845 = t2793 * t2844;
    let t2847 = 0.16081979498692535067e2_f64 * t2842 * t2845;
    let t2848 = 0.22831111111111111111e-1_f64 * t2764;
    let t2853 = t2848 + 0.11415555555555555555e-1_f64 * t2766 - 0.11415555555555555555e-1_f64 * t2773 + 0.34246666666666666666e-1_f64 * t2778 - 0.17123333333333333333e-1_f64 * t2782;
    let t2856 = t919 * t923;
    let t2859 = t922 * t307;
    (t2843, t2844, t2845, t2847, t2848, t2853, t2856, t2859)
}
