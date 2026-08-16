//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta509(t25411: f64, t27186: f64, t213: f64, t7759: f64, t25431: f64, t212: f64, t780: f64, t689: f64, t1032: f64, t1568: f64, t1955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27187, t27189, t27192, t27194, t27195, t27196, t27198, t27199) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1890(t25411, t27186, t213, t7759, t25431, t212, t780, t689, t1032, t1568, t1955);
    (t27187, t27189, t27192, t27194, t27195, t27196, t27198, t27199)
}
