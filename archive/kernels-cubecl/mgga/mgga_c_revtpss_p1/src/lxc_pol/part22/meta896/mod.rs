//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta896 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta896<F: Float>(t11675: F, t15682: F, t11711: F, t15618: F, t1043: F, t1469: F, t3133: F, t3162: F, t3115: F, t42793: F, t4906: F, t11722: F, t4834: F) -> (F, F, F, F, F, F) {
        let (t53559, t53567, t53585, t53586, t53612, t53626) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3088::<F>(t11675, t15682, t11711, t15618, t1043, t1469, t3133, t3162, t3115, t42793, t4906, t11722, t4834);
    (t53559, t53567, t53585, t53586, t53612, t53626)
}
