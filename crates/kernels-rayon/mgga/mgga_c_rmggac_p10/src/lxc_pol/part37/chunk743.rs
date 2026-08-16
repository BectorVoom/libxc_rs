//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 743/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk743(t71632: f64, t3219: f64, t7939: f64, t699: f64, t830: f64, t739: f64, t1327: f64, t640: f64, t702: f64, t7323: f64, t70071: f64, t70078: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71633 = 0.33105799917009430643e-4_f64 * t71632;
    let t71634 = t7939 * t3219;
    let t71637 = t699 * t830;
    let t71638 = t739 * t71637;
    let t71639 = 0.14635184302277988245e0_f64 * t71638;
    let t71660 = t7323 * t640 * t702 * t1327;
    let t71661 = 0.34200192530023447503e-6_f64 * t71660;
    let t71670 = 0.66671395154821946452e-1_f64 * t70071;
    let t71671 = 0.39032073591371545778e-3_f64 * t70078;
    (t71633, t71634, t71637, t71639, t71661, t71670, t71671)
}
