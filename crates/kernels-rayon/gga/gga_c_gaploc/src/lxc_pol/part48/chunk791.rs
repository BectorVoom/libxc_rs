//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 791/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk791(t10318: f64, t1397: f64, t9287: f64, t2487: f64, t2754: f64, t9438: f64, t9448: f64, t12968: f64, t34471: f64, t34286: f64, t10615: f64, t40186: f64) -> (f64, f64, f64, f64, f64) {
    let t41914 = t1397 * t10318 * t9287;
    let t41918 = t2487 * t9438 * t9448 * t2754;
    let t41947 = t34471 * t12968;
    let t41949 = t34286 * t12968;
    let t41951 = t10615 * t40186;
    (t41914, t41918, t41947, t41949, t41951)
}
