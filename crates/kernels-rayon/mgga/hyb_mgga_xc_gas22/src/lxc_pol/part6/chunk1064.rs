//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1064/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1064(t2014: f64, t3939: f64, t684: f64, t3930: f64, t3934: f64, t214: f64, t3938: f64, t675: f64, t1289: f64, t3139: f64, t1238: f64, t1318: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10182 = t684 * t2014 * t3939;
    let t10185 = t684 * t2014 * t3930;
    let t10188 = t684 * t2014 * t3934;
    let t10190 = t214 * t3938;
    let t10191 = t10190 * t675;
    let t10195 = t3139 * t1289;
    let t10199 = t1318 * t1238;
    (t10182, t10185, t10188, t10190, t10191, t10195, t10199)
}
