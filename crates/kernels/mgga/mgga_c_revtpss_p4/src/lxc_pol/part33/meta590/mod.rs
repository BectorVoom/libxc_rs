//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta590<F: Float>(t25310: F, t25331: F, t2435: F, t25339: F, t11064: F, t7086: F, t1113: F, t2411: F, t530: F, t7311: F, t2470: F, t26049: F) -> (F, F, F, F, F, F) {
        let (t93384, t93391, t93404, t94245, t94345, t94377) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2005::<F>(t25310, t25331, t2435, t25339, t11064, t7086, t1113, t2411, t530, t7311, t2470, t26049);
    (t93384, t93391, t93404, t94245, t94345, t94377)
}
