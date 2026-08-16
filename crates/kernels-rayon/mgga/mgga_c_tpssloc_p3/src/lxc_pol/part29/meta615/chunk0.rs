//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2056/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2056(t24574: f64, t24630: f64, t24605: f64, t85639: f64, t24888: f64, t24705: f64, t7327: f64, t1176: f64, t1184: f64, t24847: f64, t974: f64, t1009: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85766 = t24574 * t24630;
    let t85787 = t85639 * t24605;
    let t85789 = t24574 * t24888;
    let t85814 = t24705 * t7327;
    let t85820 = t24847 * t974 * t1176 * t1184;
    let t85821 = t460 * t1009;
    (t85766, t85787, t85789, t85814, t85820, t85821)
}
