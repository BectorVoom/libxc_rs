//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2018;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta592(t22637: f64, t81228: f64, t81326: f64, t22638: f64, t81159: f64, t22892: f64, t6891: f64, t80645: f64, t6892: f64, t81186: f64, t22674: f64, t22934: f64, t6897: f64, t22935: f64, t6883: f64, t22667: f64, t1987: f64, t81144: f64, t9537: f64, t107: f64, t835: f64, t240: f64, t656: f64, t666: f64, t2331: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81328, t81350, t81365, t81375, t81379) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2018(t22637, t81228, t81326, t22638, t81159, t22892, t6891, t80645, t6892, t81186, t22674, t22934, t6897);
        let (t81393, t81395, t81399, t81438, t81439, t81440, t81442) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2019(t22935, t6883, t22667, t1987, t81144, t9537, t107, t835, t240, t656, t666, t2331, t625);
    (t81328, t81350, t81365, t81375, t81379, t81393, t81395, t81399, t81438, t81439, t81440, t81442)
}
