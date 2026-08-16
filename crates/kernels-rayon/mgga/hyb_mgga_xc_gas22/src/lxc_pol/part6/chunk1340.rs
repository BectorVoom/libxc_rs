//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1340/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1340(t20691: f64, t20697: f64, t28794: f64, t28797: f64, t28800: f64, t28804: f64, t28808: f64, t28837: f64, t28840: f64, t28844: f64, t28847: f64, t20694: f64, t20703: f64, t20706: f64, t21057: f64, t21071: f64, t28850: f64, t28853: f64, t28856: f64, t28859: f64, t28862: f64, t28866: f64, t28872: f64) -> (f64, f64) {
    let t29263 = -0.18523555555555555555e1_f64 * t20691 + 0.34731666666666666666e0_f64 * t20697 + 0.34731666666666666667e0_f64 * t28794 - 0.41678e0_f64 * t28797 - 0.41678e0_f64 * t28800 + 0.312585e0_f64 * t28804 + 0.62517e0_f64 * t28808 + 0.312585e0_f64 * t28837 - 0.83356e0_f64 * t28840 + 0.62517e0_f64 * t28844 - 0.3529725e1_f64 * t28847;
    let t29274 = 0.6311625e0_f64 * t28850 - 0.103295e1_f64 * t28853 + 0.1549425e1_f64 * t28856 + 0.68863333333333333333e0_f64 * t28859 + 0.34731666666666666667e0_f64 * t28862 + 0.62517e0_f64 * t28866 + t21071 + 0.34731666666666666666e0_f64 * t20694 + t21057 - 0.32136222222222222222e1_f64 * t20703 + 0.68863333333333333333e0_f64 * t20706 + 0.264729375e1_f64 * t28872;
    (t29263, t29274)
}
