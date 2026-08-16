//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk931;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk932;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta246(t1868: f64, t566: f64, t198: f64, t532: f64, t1907: f64, t4147: f64, t1317: f64, t1857: f64, t1320: f64, t1468: f64, t3833: f64, t2: f64, t513: f64, t30: f64, t33: f64, t580: f64, t605: f64, t1711: f64, t3841: f64, t516: f64, t1113: f64, t162: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5537, t5541, t5542, t5546, t5548, t5549, t5552) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk931(t1868, t566, t198, t532, t1907, t4147, t1317, t1857, t1320, t1468, t3833, t2, t513);
        let (t5557, t5560, t5566) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk932(t30, t33, t5549, t5552, t580, t605, t1711, t3841, t2, t516, t1113, t162, zeta_threshold);
    (t5537, t5541, t5542, t5546, t5548, t5549, t5552, t5557, t5560, t5566)
}
