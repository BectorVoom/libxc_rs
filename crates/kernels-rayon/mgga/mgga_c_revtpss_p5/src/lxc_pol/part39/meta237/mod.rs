//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta237(t3767: f64, t5330: f64, t1248: f64, t3603: f64, t5332: f64, t3720: f64, t1774: f64, t1250: f64, t1794: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5340, t5341, t5342, t5343, t5346, t5347, t5348, t5351) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk911(t3767, t5330, t1248, t3603, t5332, t3720, t1774, t1250, t1794, t73);
    (t5340, t5341, t5342, t5343, t5346, t5347, t5348, t5351)
}
