//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1408/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1408(t20670: f64, t20671: f64, t26922: f64, t21283: f64, t2890: f64, t7035: f64, t10402: f64, t4811: f64, t10409: f64, t1441: f64, t31557: f64, t493: f64) -> (f64, f64, f64, f64, f64) {
    let t34873 = t20670 * t20671 * t26922;
    let t34874 = 0.85206502119823888168e-1_f64 * t34873;
    let t34876 = t21283 * t2890 * t7035;
    let t34877 = 0.38342925953920749676e0_f64 * t34876;
    let t34878 = t4811 * t10402;
    let t34879 = 0.51123901271894332902e0_f64 * t34878;
    let t34880 = t1441 * t10409;
    let t34881 = 0.1022478025437886658e1_f64 * t34880;
    let t34882 = t493 * t31557;
    (t34874, t34877, t34879, t34881, t34882)
}
