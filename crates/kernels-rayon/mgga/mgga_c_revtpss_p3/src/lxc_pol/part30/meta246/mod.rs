//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1097;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta246(t1312: f64, t1518: f64, t2322: f64, t4246: f64, t4248: f64, t4292: f64, t5523: f64, t670: f64, t1450: f64, t1907: f64, t198: f64, t530: f64, t1868: f64, t566: f64, t532: f64, t4147: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t5528, t5532, t5536) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1097(t1312, t1518, t2322, t4246, t4248, t4292, t5523, t670, t1450, t1907, t198, t530);
        let (t5537, t5541, t5542) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1098(t1868, t566, t198, t532, t1907, t4147);
    (t5528, t5532, t5536, t5537, t5541, t5542)
}
