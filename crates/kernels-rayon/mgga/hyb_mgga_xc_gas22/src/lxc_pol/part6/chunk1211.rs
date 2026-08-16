//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1211/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1211(t1890: f64, t7953: f64, t7966: f64, t3011: f64, t6012: f64, t19754: f64, t39: f64, t7962: f64, t7948: f64, t7942: f64, t7958: f64, t7975: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23255 = t1890 * t7953;
    let t23257 = t1890 * t7966;
    let t23284 = t6012 * t3011;
    let t23295 = t19754 * t39;
    let t23311 = t1890 * t7962;
    let t23313 = t1890 * t7948;
    let t23315 = t7942 * t7958;
    let t23317 = t1890 * t7975;
    (t23255, t23257, t23284, t23295, t23311, t23313, t23315, t23317)
}
