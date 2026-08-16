//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 923/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk923(t1252: f64, t6270: f64, t1248: f64, t6012: f64, t1890: f64, t3174: f64, t3180: f64, t3184: f64, t7942: f64, t39: f64, t6289: f64, t1238: f64, t6291: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8267 = t6270 * t1252;
    let t8288 = t6012 * t1248;
    let t8291 = 2.0_f64 / 243.0_f64 * t1890 * t3174;
    let t8293 = 2.0_f64 / 81.0_f64 * t1890 * t3180;
    let t8294 = t7942 * t3184;
    let t8296 = t6289 * t39;
    let t8297 = t6291 * t1238;
    (t8267, t8288, t8291, t8293, t8294, t8296, t8297)
}
