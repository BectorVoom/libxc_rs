//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2287/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2287(t55353: f64, t7769: f64, t16521: f64, t7467: f64, t1873: f64, t19534: f64, t3941: f64, t28017: f64, t671: f64, t20173: f64, t28899: f64, t1395: f64, t5456: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100917 = 54.0_f64 * t55353 * t7769;
    let t100921 = 27.0_f64 * t16521 * t7467;
    let t100924 = 27.0_f64 * t3941 * t1873 * t19534;
    let t100927 = 27.0_f64 * t3941 * t28017 * t671;
    let t100929 = 27.0_f64 * t20173 * t28899;
    let t100930 = t1395 * t5456;
    (t100917, t100921, t100924, t100927, t100929, t100930)
}
