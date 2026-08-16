//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1351/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1351(t2563: f64, t2610: f64, t225: f64, t2592: f64, t2710: f64, t814: f64, t252: f64, t2678: f64, t856: f64, t68: f64, t2745: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10038 = t2563 * t2610;
    let t10049 = t2592 * t225;
    let t10076 = t814 * t2710;
    let t10097 = t252 * t2678;
    let t10108 = t856 * t856;
    let t10109 = 1.0_f64 / t10108;
    let t10110 = t68 * t10109;
    let t10126 = t2745 * t870;
    (t10038, t10049, t10076, t10097, t10108, t10109, t10110, t10126)
}
