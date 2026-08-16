//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 945/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk945(t10305: f64, t951: f64, t3874: f64, t410: f64, t6514: f64, t10121: f64, t7832: f64, t10092: f64, t2970: f64, t6523: f64, t3187: f64, t1227: f64, t1245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10306 = t10305 * t951;
    let t10309 = t410 * t3874;
    let t10310 = t6514 * t10309;
    let t10311 = t7832 * t10121;
    let t10316 = t2970 * t10092;
    let t10319 = t6523 * t10309;
    let t10320 = t7832 * t3187;
    let t10323 = t1245 * t1227;
    (t10306, t10309, t10310, t10311, t10316, t10319, t10320, t10323)
}
