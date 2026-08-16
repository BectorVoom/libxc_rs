//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1250/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1250(t1523: f64, t462: f64, t7482: f64, t3616: f64, t7554: f64, t10: f64, t1107: f64, t9369: f64, t7269: f64, t7516: f64, t7242: f64, t1057: f64, t9327: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25957 = t462 * t1523 * t7482;
    let t25959 = t3616 * t7554;
    let t25962 = t9369 * t10 * t1107;
    let t25964 = t3616 * t7269;
    let t25966 = t3616 * t7516;
    let t25968 = t3616 * t7242;
    let t25973 = t1057 * t9327;
    (t25957, t25959, t25962, t25964, t25966, t25968, t25973)
}
