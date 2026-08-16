//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta70 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk422;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk423;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk424;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk425;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta70<F: Float>(t212: F, t555: F, t225: F, t561: F, t689: F, t556: F, t786: F, t72: F, t686: F, t535: F, t795: F, t159: F, t540: F, t216: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1357 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk422::<F>(t212, t555);
        let t1358 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk423::<F>(t225, t561);
        let (t1359, t1361, t1362, t1363, t1364) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk424::<F>(t1357, t1358, t689, t556, t786, t561, t72, t686);
        let (t1366, t1368, t1369, t1370) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk425::<F>(t1362, t1364, t535, t795, t159, t540, t216);
    (t1357, t1358, t1359, t1361, t1362, t1363, t1364, t1366, t1368, t1369, t1370)
}
