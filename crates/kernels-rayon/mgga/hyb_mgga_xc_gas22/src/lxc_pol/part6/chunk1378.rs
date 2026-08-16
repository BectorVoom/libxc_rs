//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1378/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1378(t29757: f64, t29760: f64, t29788: f64, t29822: f64, t29825: f64, t29827: f64, t29833: f64, t29836: f64, t29839: f64, t29842: f64, t29844: f64, t29846: f64) -> f64 {
    let t29945 = 0.6311625e0_f64 * t29822 - 0.3529725e1_f64 * t29825 + 0.6311625e0_f64 * t29827 + 0.68863333333333333333e0_f64 * t29757 - 0.103295e1_f64 * t29760 + 0.1549425e1_f64 * t29788 - 0.6618234375e1_f64 * t29833 + 0.264729375e1_f64 * t29836 + 0.2366859375e0_f64 * t29839 - 0.157790625e0_f64 * t29842 + 0.264729375e1_f64 * t29844 - 0.3529725e1_f64 * t29846;
    t29945
}
