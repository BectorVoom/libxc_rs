//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1807;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1808;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1809;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta500(t3596: f64, t3598: f64, t3594: f64, t7616: f64, t1230: f64, t7623: f64, t3636: f64, t7624: f64, t3704: f64, t7618: f64, t479: f64, sigma2: f64, t3089: f64, t1285: f64, t3717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26843, t26844, t26848, t26849, t26852, t26855, t26863, t26865) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1807(t3596, t3598, t3594, t7616, t1230, t7623, t3636, t7624, t3704, t7618, t479, sigma2);
        let t26866 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1808(t26865, t3089);
        let t26867 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1809(t1285, t26866);
        let t26870 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1810(t26866, t3717);
    (t26843, t26844, t26848, t26849, t26852, t26855, t26863, t26865, t26866, t26867, t26870)
}
