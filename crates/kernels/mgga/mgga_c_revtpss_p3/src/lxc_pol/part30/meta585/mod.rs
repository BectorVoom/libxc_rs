//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2040;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta585<F: Float>(t545: F, t94667: F, t25875: F, t25925: F, t686: F, t72: F, t25894: F, t25950: F, t25953: F, t26069: F, t94407: F, t1445: F, t25912: F, t689: F, t7282: F, t9646: F, t2022: F, t22: F, t25937: F, t93139: F, t1955: F, t25920: F, t4075: F, t2435: F, t26061: F, t1385: F, t7274: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94669, t94672, t94674, t94675, t94677, t94682, t94694) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2040::<F>(t545, t94667, t25875, t25925, t686, t72, t25894, t25950, t25953, t26069, t94407, t1445, t25912, t689);
        let (t94700, t94703, t94705, t94714, t94716) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2041::<F>(t7282, t9646, t2022, t22, t25937, t93139, t1955, t25920, t4075, t2435, t26061, t1385, t7274);
    (t94669, t94672, t94674, t94675, t94677, t94682, t94694, t94700, t94703, t94705, t94714, t94716)
}
