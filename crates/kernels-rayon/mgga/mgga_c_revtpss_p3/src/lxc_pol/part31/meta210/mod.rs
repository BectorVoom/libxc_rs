//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk956;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta210(t30: f64, t1317: f64, t1857: f64, t1320: f64, t1468: f64, t3833: f64, t2: f64, t513: f64, t580: f64, t605: f64, t1711: f64, t3841: f64, t516: f64, zeta_threshold: f64, t33: f64, t1113: f64, t162: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5545, t5546, t5547, t5548, t5549, t5556, t5557, t5560) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk956(t30, t1317, t1857, t1320, t1468, t3833, t2, t513, t580, t605, t1711, t3841, t516, zeta_threshold);
        let t5566 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk957(t33, t1113, t5557, t5560, t580, t162, t5556, zeta_threshold);
    (t5545, t5546, t5547, t5548, t5549, t5557, t5566)
}
