//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1297/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1297(t3984: f64, t763: f64, t10159: f64, t2014: f64, t684: f64, t10505: f64, t10509: f64, t3188: f64, t10322: f64, t1890: f64, t10317: f64, t10306: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28136 = t763 * t3984;
    let t28150 = t684 * t2014 * t10159;
    let t28153 = t684 * t2014 * t10505;
    let t28156 = t684 * t2014 * t10509;
    let t28162 = t3188 * t3188;
    let t28183 = t1890 * t10322;
    let t28185 = t1890 * t10317;
    let t28187 = t1890 * t10306;
    (t28136, t28150, t28153, t28156, t28162, t28183, t28185, t28187)
}
