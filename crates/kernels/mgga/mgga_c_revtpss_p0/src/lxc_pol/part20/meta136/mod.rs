//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta136<F: Float>(t3682: F, t461: F, t1226: F, t140: F, t1222: F, t1225: F, t2258: F, t1012: F, t1224: F, t3367: F, t2251: F, t1121: F, t404: F) -> (F, F, F, F, F, F, F, F) {
        let (t3684, t3686, t3688, t3689, t3692, t3693, t3694, t3698) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk765::<F>(t3682, t461, t1226, t140, t1222, t1225, t2258, t1012, t1224, t3367, t2251, t1121, t404);
    (t3684, t3686, t3688, t3689, t3692, t3693, t3694, t3698)
}
