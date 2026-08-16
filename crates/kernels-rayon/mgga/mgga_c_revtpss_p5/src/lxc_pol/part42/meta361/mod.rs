//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1176;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta361(t1222: f64, t17472: f64, t1012: f64, t13026: f64, t1263: f64, t5245: f64, t1234: f64, t5390: f64, t3704: f64, t5293: f64, t3172: f64, t5286: f64, t1247: f64, t3707: f64, t5292: f64, t12268: f64, t3617: f64, t3708: f64, t5265: f64, t1260: f64, t5326: f64, t5274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17474, t17475, t17500, t17505, t17509, t17544) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1176(t1222, t17472, t1012, t13026, t1263, t5245, t1234, t5390, t3704, t5293, t3172, t5286);
        let (t17546, t17547, t17550, t17556, t17569, t17593) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1177(t1247, t17544, t3707, t5292, t12268, t3617, t3708, t5265, t1260, t5326, t3704, t5274);
    (t17474, t17475, t17500, t17505, t17509, t17546, t17547, t17550, t17556, t17569, t17593)
}
