//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta340(t12851: f64, t461: f64, t1209: f64, t3766: f64, t5330: f64, t1214: f64, t3603: f64, t11772: f64, t3623: f64, t3717: f64, t1263: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12853, t12854, t12855, t12856, t12865, t12866, t12879) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1351(t12851, t461, t1209, t3766, t5330, t1214, t3603, t11772, t3623, t3717, t1263, t675);
    (t12853, t12854, t12855, t12856, t12865, t12866, t12879)
}
