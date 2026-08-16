//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2336/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2336(t28017: f64, t3941: f64, t671: f64, t20173: f64, t28899: f64, t1395: f64, t5456: f64, t1873: f64, t20162: f64, t6534: f64, t26545: f64, t33185: f64) -> (f64, f64, f64, f64, f64) {
    let t100927 = 27.0_f64 * t3941 * t28017 * t671;
    let t100929 = 27.0_f64 * t20173 * t28899;
    let t100930 = t1395 * t5456;
    let t100932 = 27.0_f64 * t100930 * t1873;
    let t100934 = 0.135e2_f64 * t20162 * t6534;
    let t100936 = 54.0_f64 * t33185 * t26545;
    (t100927, t100929, t100932, t100934, t100936)
}
