//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 587/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk587(t376: f64, t5921: f64, t89: f64, t1882: f64, t5886: f64, t5866: f64, t5875: f64, t1366: f64, t8232: f64, t1378: f64, t2178: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23930 = t376 * t5921;
    let t23931 = t89 * t23930;
    let t23943 = t1882 * t5886;
    let t23945 = t1882 * t5866;
    let t23947 = t1882 * t5875;
    let t23950 = 4.0_f64 / 27.0_f64 * t8232 * t1366;
    let t23997 = t1378 * t2178;
    (t23930, t23931, t23943, t23945, t23947, t23950, t23997)
}
