//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta441<F: Float>(t1132: F, t20337: F, t1145: F, t20318: F, t141: F, t20302: F, t3417: F, t20298: F, t20310: F, t20306: F, t12327: F, t6442: F) -> (F, F, F, F, F, F, F) {
        let (t20338, t20341, t20344, t20347, t20350, t20353, t20356) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1610::<F>(t1132, t20337, t1145, t20318, t141, t20302, t3417, t20298, t20310, t20306, t12327, t6442);
    (t20338, t20341, t20344, t20347, t20350, t20353, t20356)
}
