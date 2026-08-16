//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 899/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk899(t112660: f64, t1880: f64, t6572: f64, t23218: f64, t30663: f64, t30657: f64, t6547: f64, t22986: f64, t23270: f64, t30633: f64, t87036: f64, t30671: f64) -> (f64, f64, f64, f64, f64) {
    let t112663 = 0.3289868133696452873e-1_f64 * t1880 * t112660 * t6572;
    let t112666 = 0.16449340668482264365e-1_f64 * t1880 * t30663 * t23218;
    let t112667 = t6547 * t30657;
    let t112668 = 0.76763589786250567036e-1_f64 * t112667;
    let t112672 = 0.13159472534785811492e0_f64 * t22986 * t23270 * t30633 * t87036;
    let t112673 = t6547 * t30671;
    (t112663, t112666, t112668, t112672, t112673)
}
