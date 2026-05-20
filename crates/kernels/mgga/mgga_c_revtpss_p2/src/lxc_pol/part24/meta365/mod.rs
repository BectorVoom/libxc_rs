//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta365<F: Float>(t5087: F, t6449: F, t12254: F, t24228: F, t141: F, t1145: F, t24244: F, t16706: F, t16876: F, t20276: F, t20278: F, t20280: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24265: F) -> (F, F, F, F, F, F) {
        let (t24267, t24271, t24272, t24274, t24275, t24285) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1246::<F>(t5087, t6449, t12254, t24228, t141, t1145, t24244, t16706, t16876, t20276, t20278, t20280, t20283, t20285, t20287, t24230, t24234, t24265);
    (t24267, t24271, t24272, t24274, t24275, t24285)
}
