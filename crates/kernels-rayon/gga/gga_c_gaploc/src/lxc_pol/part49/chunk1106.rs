//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1106/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1106(t40986: f64, t40989: f64, t13883: f64, t1991: f64, t590: f64, t13870: f64, t1890: f64, t1966: f64, t12240: f64, t2679: f64, t9800: f64, t3720: f64, t5241: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47155 = 0.38342925953920749677e0_f64 * t40986;
    let t47157 = 0.72851559312449424385e1_f64 * t40989;
    let t47160 = 0.51123901271894332902e0_f64 * t1991 * t13883 * t590;
    let t47164 = 0.51123901271894332902e0_f64 * t1966 * t1890 * t13870 * t590;
    let t47166 = t9800 * t12240 * t2679;
    let t47168 = t5241 * t3720;
    (t47155, t47157, t47160, t47164, t47166, t47168)
}
