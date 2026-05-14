//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1277/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1277<F: Float>(t29821: F, t29848: F, t29882: F, t29911: F, t21393: F, t21396: F, t21427: F, t21430: F, t21433: F, t21638: F, t21641: F, t25214: F, t25217: F, t25220: F, t29819: F, t29757: F, t29760: F, t29788: F, t29822: F, t29825: F, t29827: F, t29833: F, t29836: F, t29839: F, t29842: F, t29844: F, t29846: F) -> (F, F, F) {
    let t29913 = t29821 + t29848 + t29882 + t29911;
    let t29932 = t21638 - 0.32136222222222222222e1 * t21393 + 0.68863333333333333333e0 * t21396 + t21641 + 0.34731666666666666666e0 * t21430 - 0.18523555555555555555e1 * t21427 + 0.34731666666666666666e0 * t21433 - 0.32136222222222222223e1 * t25214 + 0.27545333333333333334e1 * t25217 - 0.103295e1 * t25220 + 0.3529725e1 * t29819;
    let t29945 = 0.6311625e0 * t29822 - 0.3529725e1 * t29825 + 0.6311625e0 * t29827 + 0.68863333333333333333e0 * t29757 - 0.103295e1 * t29760 + 0.1549425e1 * t29788 - 0.6618234375e1 * t29833 + 0.264729375e1 * t29836 + 0.2366859375e0 * t29839 - 0.157790625e0 * t29842 + 0.264729375e1 * t29844 - 0.3529725e1 * t29846;
    (t29913, t29932, t29945)
}
