//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1853;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1854;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta498<F: Float>(t3678: F, t7613: F, t3685: F, t7607: F, t3596: F, t3598: F, t3594: F, t1238: F, t26817: F, t26821: F, t26822: F, t26824: F, t26827: F, t3606: F, t3663: F, t3674: F, t3689: F, t3694: F, t3701: F, t484: F, sigma2: F, t7616: F, t1230: F, t7623: F, t3636: F, t7624: F, t3704: F, t7618: F, t479: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26832, t26836, t26842, t26843, t26844, t26847) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1853::<F>(t3678, t7613, t3685, t7607, t3596, t3598, t3594, t1238, t26817, t26821, t26822, t26824, t26827, t3606, t3663, t3674, t3689, t3694, t3701, t484, sigma2);
        let (t26848, t26849, t26852) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1854::<F>(t3598, t7616, t3594, t1230, t7623);
        let (t26855, t26863, t26865) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1855::<F>(t3636, t7624, t3704, t7618, t479, sigma2);
    (t26832, t26836, t26842, t26843, t26844, t26847, t26848, t26849, t26852, t26855, t26863, t26865)
}
