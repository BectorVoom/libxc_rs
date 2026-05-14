//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1247/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1247<F: Float>(t20691: F, t20697: F, t28794: F, t28797: F, t28800: F, t28804: F, t28808: F, t28837: F, t28840: F, t28844: F, t28847: F, t20694: F, t20703: F, t20706: F, t21057: F, t21071: F, t28850: F, t28853: F, t28856: F, t28859: F, t28862: F, t28866: F, t28872: F) -> (F, F) {
    let t29263 = -0.18523555555555555555e1 * t20691 + 0.34731666666666666666e0 * t20697 + 0.34731666666666666667e0 * t28794 - 0.41678e0 * t28797 - 0.41678e0 * t28800 + 0.312585e0 * t28804 + 0.62517e0 * t28808 + 0.312585e0 * t28837 - 0.83356e0 * t28840 + 0.62517e0 * t28844 - 0.3529725e1 * t28847;
    let t29274 = 0.6311625e0 * t28850 - 0.103295e1 * t28853 + 0.1549425e1 * t28856 + 0.68863333333333333333e0 * t28859 + 0.34731666666666666667e0 * t28862 + 0.62517e0 * t28866 + t21071 + 0.34731666666666666666e0 * t20694 + t21057 - 0.32136222222222222222e1 * t20703 + 0.68863333333333333333e0 * t20706 + 0.264729375e1 * t28872;
    (t29263, t29274)
}
