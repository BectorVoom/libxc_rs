//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta379<F: Float>(t1053: F, t4857: F, t1663: F, t371: F, t676: F, t1025: F, t11922: F, t4901: F, t4899: F, t3172: F, t4874: F, t3127: F) -> (F, F, F, F, F, F, F) {
        let (t15745, t15749, t15750, t15752, t15754, t15769, t15771) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1416::<F>(t1053, t4857, t1663, t371, t676, t1025, t11922, t4901, t4899, t3172, t4874, t3127);
    (t15745, t15749, t15750, t15752, t15754, t15769, t15771)
}
