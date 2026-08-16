//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2040;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta585(t545: f64, t94667: f64, t25875: f64, t25925: f64, t686: f64, t72: f64, t25894: f64, t25950: f64, t25953: f64, t26069: f64, t94407: f64, t1445: f64, t25912: f64, t689: f64, t7282: f64, t9646: f64, t2022: f64, t22: f64, t25937: f64, t93139: f64, t1955: f64, t25920: f64, t4075: f64, t2435: f64, t26061: f64, t1385: f64, t7274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94669, t94672, t94674, t94675, t94677, t94682, t94694) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2040(t545, t94667, t25875, t25925, t686, t72, t25894, t25950, t25953, t26069, t94407, t1445, t25912, t689);
        let (t94700, t94703, t94705, t94714, t94716) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2041(t7282, t9646, t2022, t22, t25937, t93139, t1955, t25920, t4075, t2435, t26061, t1385, t7274);
    (t94669, t94672, t94674, t94675, t94677, t94682, t94694, t94700, t94703, t94705, t94714, t94716)
}
