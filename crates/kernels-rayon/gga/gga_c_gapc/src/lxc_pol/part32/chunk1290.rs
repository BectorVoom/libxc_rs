//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1290/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1290(t10346: f64, t134: f64, t2207: f64, t35834: f64, t10301: f64, t2580: f64, t9497: f64, t17874: f64, t35382: f64, t35766: f64, t10237: f64, t3729: f64) -> (f64, f64, f64, f64) {
    let t35875 = t10346 * t2207 * t134 * t35834;
    let t35878 = t10301 * t2580 * t9497;
    let t35881 = t35766 * t35382 * t17874;
    let t35883 = t10237 * t3729;
    (t35875, t35878, t35881, t35883)
}
