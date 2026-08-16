//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1318/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1318(t10402: f64, t11034: f64, t11037: f64, t2402: f64, t973: f64, t999: f64, t9277: f64, t972: f64, t10263: f64, t3139: f64, t1030: f64, t10477: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42541 = t11034 * t10402;
    let t42546 = t11037 * t10402;
    let t42552 = t973 * t2402 * t999;
    let t42554 = t9277 * t972;
    let t42557 = t10263 * t3139;
    let t42559 = t1030 * t10477;
    (t42541, t42546, t42552, t42554, t42557, t42559)
}
