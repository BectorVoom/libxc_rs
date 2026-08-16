//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2872/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2872(t49072: f64, t49240: f64, t912: f64, t13727: f64, t14382: f64, t14385: f64, t49489: f64, t13520: f64, t14392: f64, t14396: f64, t49274: f64, t2836: f64, t2842: f64, t5695: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60033 = 0.2069040516770936012e4_f64 * t49240 * t49072 * t912;
    let t60035 = 4.0_f64 * t13727 * t14382;
    let t60037 = 0.19298375398431042081e3_f64 * t49489 * t14385;
    let t60039 = 0.32163958997385070134e2_f64 * t13520 * t14392;
    let t60041 = 0.1034520258385468006e4_f64 * t49274 * t14396;
    let t60044 = 6.0_f64 * t2842 * t5695 * t2836;
    (t60033, t60035, t60037, t60039, t60041, t60044)
}
