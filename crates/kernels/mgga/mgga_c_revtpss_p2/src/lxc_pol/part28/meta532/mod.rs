//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta532<F: Float>(t28076: F, t72: F, t1927: F, t6977: F, t7715: F, t6973: F, t7719: F, t4237: F, t76: F, t1926: F, t13269: F, t38: F) -> (F, F, F, F, F, F, F) {
        let (t28077, t28078, t28081, t28086, t28089, t28090, t28093) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1974::<F>(t28076, t72, t1927, t6977, t7715, t6973, t7719, t4237, t76, t1926, t13269, t38);
    (t28077, t28078, t28081, t28086, t28089, t28090, t28093)
}
