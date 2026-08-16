//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta506<F: Float>(t27137: F, t651: F, t7235: F, t7935: F, t1353: F, t1907: F, t8717: F, t25082: F, t1962: F, t198: F, t205: F) -> (F, F, F, F, F, F) {
        let (t27139, t27152, t27153, t27154, t27156, t27158) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1825::<F>(t27137, t651, t7235, t7935, t1353, t1907, t8717, t25082, t1962, t198, t205);
    (t27139, t27152, t27153, t27154, t27156, t27158)
}
