//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 862/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk862(t3645: f64, t443: f64, t1004: f64, t3102: f64, t3062: f64, t3077: f64, t1160: f64, t180: f64, t3101: f64, t407: f64, t12265: f64, t150: f64) -> (f64, f64, f64, f64, f64) {
    let t12282 = t3645 * t443;
    let t12285 = 0.26341796731742046395e1_f64 * t1004 * t3102;
    let t12286 = t3077 * t3062;
    let t12290 = t1160 * t180 * t3101 * t407;
    let t12295 = t12265 * t150;
    (t12282, t12285, t12286, t12290, t12295)
}
