//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk910;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta210(t30: f64, t189: f64, t5566: f64, t512: f64, t1856: f64, t749: f64, t177: f64, t762: f64, t1468: f64, t3874: f64, t1344: f64, t2: f64, t580: f64, t605: f64, zeta_threshold: f64, t33: f64, t1711: f64, t3881: f64, t1348: f64, t1113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5567, t5568, t5569, t5570, t5571, t5572, t5573, t5574, t5581) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk910(t30, t189, t5566, t512, t1856, t749, t177, t762, t1468, t3874, t1344, t2, t580, t605, zeta_threshold);
        let (t5582, t5591) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk911(t33, t1711, t3881, t1348, t2, t1113, t580, t5581, zeta_threshold);
    (t5567, t5568, t5569, t5570, t5571, t5572, t5573, t5574, t5582, t5591)
}
