//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta283<F: Float>(t1337: F, t9586: F, t4146: F, t565: F, t1333: F, t3860: F, t4144: F, t4147: F, t30: F, t513: F, t33: F, t516: F) -> (F, F, F, F, F, F, F, F) {
        let (t9588, t9593, t9597, t9599, t9603, t9605, t9615, t9617) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1695::<F>(t1337, t9586, t4146, t565, t1333, t3860, t4144, t4147, t30, t513, t33, t516);
    (t9588, t9593, t9597, t9599, t9603, t9605, t9615, t9617)
}
