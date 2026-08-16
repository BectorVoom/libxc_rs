//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk838;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta157(t421: f64, t3385: f64, t3433: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t1156: f64, t1160: f64, t1159: f64, t431: f64, t426: f64, t1168: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3434, t3435, t3436, t3438, t3439, t3444, t3447, t3450) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk838(t421, t3385, t3433, t3356, t3358, t3365, t3370, t3374, t1156, t1160, t1159, t431);
        let (t3451, t3452, t3453) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk839(t3450, t426, t1168);
    (t3434, t3435, t3436, t3438, t3439, t3444, t3447, t3451, t3452, t3453)
}
