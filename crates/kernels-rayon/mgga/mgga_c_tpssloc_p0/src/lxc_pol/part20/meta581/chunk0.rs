//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2148/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2148(t10883: f64, t10884: f64, t248: f64, t3101: f64, t10473: f64, t361: f64, t363: f64, t42342: f64, t42345: f64, t3131: f64, t3047: f64, t3077: f64) -> (f64, f64, f64, f64, f64) {
    let t43285 = t10883 * t248 * t3101 * t10884;
    let t43288 = 1.0_f64 / t10473 / t361;
    let t43291 = t42342 * t43288 * t363 * t42345;
    let t43292 = t3131 * t3131;
    let t43298 = t3077 * t3047;
    (t43285, t43288, t43291, t43292, t43298)
}
