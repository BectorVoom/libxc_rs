//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 452/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk452(t972: f64, t195: f64, t896: f64, t311: f64, t668: f64, t761: f64, t285: f64, t5: f64, t1033: f64, t277: f64, t1971: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2470 = t972 * t972;
    let t2473 = t896 * t195;
    let t2474 = t311 * t2473;
    let t2477 = t761 * t668;
    let t2480 = t285 * t5;
    let t2481 = t2480 * t1033;
    let t2482 = t277 * t2481;
    let t2483 = t1971 * t333;
    (t2470, t2474, t2477, t2480, t2482, t2483)
}
