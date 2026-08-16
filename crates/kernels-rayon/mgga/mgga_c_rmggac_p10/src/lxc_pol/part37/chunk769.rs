//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 769/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk769(t15391: f64, t68541: f64, t15067: f64, t68490: f64, t15376: f64, t68524: f64, t14117: f64, t21708: f64, t9137: f64, t15336: f64, t68528: f64, t217: f64, t3119: f64, t597: f64, t7715: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73922 = t68541 * t15391;
    let t73924 = t68490 * t15067;
    let t73926 = t68524 * t15376;
    let t73929 = t21708 * t14117 * t9137;
    let t73931 = t68528 * t15336;
    let t73935 = t217 * t597 * t7715 * t3119;
    (t73922, t73924, t73926, t73929, t73931, t73935)
}
