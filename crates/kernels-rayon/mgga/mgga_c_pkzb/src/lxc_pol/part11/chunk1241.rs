//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1241/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1241(t21184: f64, t9225: f64, t17351: f64, t17405: f64, t17454: f64, t20705: f64, t25633: f64, t25636: f64, t25734: f64, t25740: f64, t25747: f64, t25750: f64, t25767: f64, t30284: f64, t30287: f64, t30289: f64, t30291: f64, t30294: f64, t30296: f64, t30309: f64, t30311: f64) -> (f64, f64) {
    let t30502 = 18.0_f64 * t21184 * t9225;
    let t30525 = -0.73586666666666666666e0_f64 * t17405 - 0.28179666666666666667e1_f64 * t20705 + 0.258925e1_f64 * t30289 + 0.16504875e0_f64 * t30291 + 0.58258125e1_f64 * t30294 - 0.1237865625e0_f64 * t30296 + t17454 - 0.93932222222222222223e0_f64 * t17351 + 0.12077e1_f64 * t25633 - 0.905775e0_f64 * t25636 + 0.82785e0_f64 * t25734 - 0.301925e0_f64 * t30284 + 0.905775e0_f64 * t30287 - 0.99342e0_f64 * t25740 - 0.49671e0_f64 * t25747 - 0.49671e0_f64 * t25750 + 0.82785e0_f64 * t25767 + 0.6189328125e-1_f64 * t30309 - 0.1237865625e0_f64 * t30311;
    (t30502, t30525)
}
