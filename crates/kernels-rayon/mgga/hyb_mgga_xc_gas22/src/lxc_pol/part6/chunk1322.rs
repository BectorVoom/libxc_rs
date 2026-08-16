//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1322/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1322(t2213: f64, t238: f64, t4131: f64, t3309: f64, t242: f64, t2195: f64, t4121: f64, t6585: f64, t20625: f64, t20689: f64, t20694: f64, t20703: f64, t20706: f64, t28850: f64, t28853: f64, t28856: f64, t28859: f64) -> (f64, f64, f64, f64) {
    let t28862 = t238 * t2213 * t4131;
    let t28864 = t3309 * t3309;
    let t28866 = t238 * t242 * t28864;
    let t28872 = t6585 * t4121 * t2195;
    let t28874 = 0.16504875e0_f64 * t28850 - 0.60385e0_f64 * t28853 + 0.905775e0_f64 * t28856 + 0.40256666666666666667e0_f64 * t28859 + 0.27595e0_f64 * t28862 + 0.49671e0_f64 * t28866 + t20689 + 0.27595e0_f64 * t20694 + t20625 - 0.18786444444444444445e1_f64 * t20703 + 0.40256666666666666667e0_f64 * t20706 + 0.19419375e1_f64 * t28872;
    (t28862, t28866, t28872, t28874)
}
