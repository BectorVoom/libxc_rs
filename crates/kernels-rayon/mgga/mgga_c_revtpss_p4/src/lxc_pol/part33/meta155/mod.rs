//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk791;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk792;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta155(t4086: f64, t555: f64, t786: f64, t1398: f64, t675: f64, t268: f64, t543: f64, t1419: f64, t72: f64, t1432: f64, t686: f64, t1433: f64, t2470: f64, t3999: f64, t1385: f64, t198: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4100, t4101) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk791(t4086, t555, t786);
        let (t4102, t4104, t4105, t4107, t4109, t4113, t4114) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk792(t1398, t675, t268, t543, t4101, t1419, t72, t1432, t686, t1433, t2470, t3999, t555);
        let (t4118, t4139) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk793(t1385, t1419, t198, t531);
    (t4100, t4101, t4102, t4104, t4105, t4107, t4109, t4113, t4114, t4118, t4139)
}
