//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1341/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1341(t28877: f64, t28880: f64, t28883: f64, t28885: f64, t28887: f64, t28890: f64, t28892: f64, t28894: f64, t28896: f64, t28899: f64, t28901: f64, t28904: f64) -> f64 {
    let t29288 = 0.2366859375e0_f64 * t28877 - 0.157790625e0_f64 * t28880 - 0.6618234375e1_f64 * t28883 + 0.264729375e1_f64 * t28885 - 0.3529725e1_f64 * t28887 - 0.3529725e1_f64 * t28890 - 0.17648625e1_f64 * t28892 - 0.157790625e0_f64 * t28894 + 0.6311625e0_f64 * t28896 + 0.6311625e0_f64 * t28899 + 0.31558125e0_f64 * t28901 - 0.6311625e0_f64 * t28904;
    t29288
}
