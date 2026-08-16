//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1040;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta156<F: Float>(t1235: F, t3678: F, t221: F, t462: F, t696: F, t461: F, t1226: F, t140: F, t1222: F, t1225: F, t2258: F, t1012: F, t1224: F, t3367: F, t2251: F, t1121: F, t404: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3679, t3682, t3684, t3685, t3686, t3688, t3689) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1040::<F>(t1235, t3678, t221, t462, t696, t461, t1226, t140, t1222, t1225, t2258, t1012);
        let (t3692, t3693, t3694, t3698) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1041::<F>(t1224, t3367, t2251, t1012, t1121, t404);
    (t3679, t3682, t3684, t3685, t3686, t3688, t3689, t3692, t3693, t3694, t3698)
}
