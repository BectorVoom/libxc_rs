//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2152/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2152(t1058: f64, t3068: f64, t3087: f64, t363: f64, t11065: f64, t42387: f64, t10250: f64, t2970: f64, t973: f64, t10195: f64, t10231: f64, t1005: f64, t10375: f64) -> (f64, f64, f64, f64, f64) {
    let t43358 = t1058 * t363 * t3087 * t3068;
    let t43361 = t11065 * t42387;
    let t43374 = t973 * t2970 * t10250;
    let t43377 = t973 * t10231 * t10195;
    let t43382 = t1005 * t10375;
    (t43358, t43361, t43374, t43377, t43382)
}
