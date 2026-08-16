//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1853;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1854;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta498(t3678: f64, t7613: f64, t3685: f64, t7607: f64, t3596: f64, t3598: f64, t3594: f64, t1238: f64, t26817: f64, t26821: f64, t26822: f64, t26824: f64, t26827: f64, t3606: f64, t3663: f64, t3674: f64, t3689: f64, t3694: f64, t3701: f64, t484: f64, sigma2: f64, t7616: f64, t1230: f64, t7623: f64, t3636: f64, t7624: f64, t3704: f64, t7618: f64, t479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26832, t26836, t26842, t26843, t26844, t26847) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1853(t3678, t7613, t3685, t7607, t3596, t3598, t3594, t1238, t26817, t26821, t26822, t26824, t26827, t3606, t3663, t3674, t3689, t3694, t3701, t484, sigma2);
        let (t26848, t26849, t26852) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1854(t3598, t7616, t3594, t1230, t7623);
        let (t26855, t26863, t26865) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1855(t3636, t7624, t3704, t7618, t479, sigma2);
    (t26832, t26836, t26842, t26843, t26844, t26847, t26848, t26849, t26852, t26855, t26863, t26865)
}
