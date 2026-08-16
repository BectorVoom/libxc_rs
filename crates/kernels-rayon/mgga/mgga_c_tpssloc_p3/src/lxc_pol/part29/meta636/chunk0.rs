//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2088/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2088(t1877: f64, t2219: f64, t6666: f64, t25353: f64, t2752: f64, t25213: f64, t6547: f64, t22986: f64, t23270: f64, t25053: f64, t2553: f64, t4119: f64, t857: f64) -> (f64, f64, f64, f64, f64) {
    let t86835 = 2.0_f64 * t1877 * t6666 * t2219;
    let t86836 = t25353 * t2752;
    let t86843 = t6547 * t25213;
    let t86844 = 0.38381794893125283518e-1_f64 * t86843;
    let t86847 = t22986 * t23270 * t25053 * t2553;
    let t86849 = t857 * t4119;
    (t86835, t86836, t86844, t86847, t86849)
}
