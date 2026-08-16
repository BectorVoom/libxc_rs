//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta503(t1209: f64, t7627: f64, t460: f64, t2142: f64, t3555: f64, t1204: f64, t26936: f64, t3801: f64, t7669: f64, t12587: f64, t2155: f64, t116: f64, t7583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26999, t27008, t27011, t27020, t27025, t27037, t27041, t27060) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1822(t1209, t7627, t460, t2142, t3555, t1204, t26936, t3801, t7669, t12587, t2155, t116, t7583);
    (t26999, t27008, t27011, t27020, t27025, t27037, t27041, t27060)
}
