//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 558/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk558(t3868: f64, t865: f64, t1212: f64, t310: f64, t447: f64, t150: f64, t443: f64, t848: f64, t1264: f64, t322: f64, t449: f64, t316: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3869 = t3868 * t865;
    let t3871 = t310 * t1212;
    let t3873 = t447 * t447;
    let t3874 = 1.0_f64 / t3873;
    let t3875 = t150 * t3874;
    let t3880 = t848 * t443;
    let t3882 = t322 * t1264;
    let t3883 = t449 * t3882;
    let t3884 = t316 * t3883;
    (t3869, t3871, t3873, t3874, t3875, t3880, t3883, t3884)
}
