//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1945;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta555(t30: f64, t6079: f64, t1468: f64, t1583: f64, t6075: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25445: f64, t27368: f64, t29592: f64, t29599: f64, t29602: f64, t29606: f64, t29705: f64, t4541: f64, t5824: f64, t7091: f64, t7749: f64, t7783: f64, t7787: f64, t5966: f64, t1544: f64, t198: f64, t207: f64, t29598: f64, t29704: f64, t5962: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t29713, t29716, t29719, t29726) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1945(t30, t6079, t1468, t1583, t6075, t1940, t1963, t2403, t25206, t25445, t27368, t29592, t29599, t29602, t29606, t29705, t4541, t5824, t7091, t7749, t7783, t7787);
        let (t29907, t29930) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1946(t1963, t5966, t1544, t1583, t1940, t198, t207, t2403, t25445, t27368, t29598, t29704, t4541, t5962, t6075, t6079, t7091, t7783, t892);
    (t29713, t29716, t29719, t29726, t29907, t29930)
}
