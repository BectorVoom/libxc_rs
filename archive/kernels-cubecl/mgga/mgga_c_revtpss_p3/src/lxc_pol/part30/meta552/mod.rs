//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta552<F: Float>(t13026: F, t65: F, t2246: F, t4171: F, t10308: F, t1466: F, t13267: F, t602: F, t4343: F, t890: F, t1544: F, t2408: F) -> (F, F, F, F, F, F) {
        let (t57549, t60221, t60224, t60248, t61102, t61155) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1991::<F>(t13026, t65, t2246, t4171, t10308, t1466, t13267, t602, t4343, t890, t1544, t2408);
    (t57549, t60221, t60224, t60248, t61102, t61155)
}
