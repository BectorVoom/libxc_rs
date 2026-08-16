//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk667;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk668;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk669;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta140(t3140: f64, t342: f64, t1034: f64, t358: f64, t360: f64, t368: f64, t335: f64, t365: f64, t1043: f64, t373: f64, t73: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3141, t3143, t3145, t3147, t3148, t3149, t3150, t3151) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk667(t3140, t342, t1034, t358, t360, t368, t335, t365, t1043);
        let (t3152, t3153) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk668(t3151, t373, t73);
        let t3154 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk669(t357);
    (t3141, t3143, t3145, t3147, t3148, t3149, t3150, t3151, t3152, t3153, t3154)
}
