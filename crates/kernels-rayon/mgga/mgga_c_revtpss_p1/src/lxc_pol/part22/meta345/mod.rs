//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1826;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1827;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta345(t11626: f64, t3154: f64, t357: f64, t11249: f64, t3129: f64, t3172: f64, t3127: f64, t3135: f64, t1041: f64, t1024: f64, t3105: f64, t3151: f64, t3153: f64, t905: f64, t606: f64, t1052: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11627, t11631) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1826(t11626, t3154, t357);
        let (t11632, t11643, t11644, t11648, t11649, t11656) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1827(t11249, t11631, t3129, t3172, t3127, t3135, t1041, t1024, t3105);
        let (t11659, t11660, t11661, t11670) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1828(t3151, t3153, t3154, t905, t606, t1052, t360);
    (t11627, t11631, t11632, t11643, t11644, t11648, t11649, t11656, t11659, t11660, t11661, t11670)
}
