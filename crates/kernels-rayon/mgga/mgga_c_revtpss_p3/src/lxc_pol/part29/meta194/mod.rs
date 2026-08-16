//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta194 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk894;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk895;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk896;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta194(t1466: f64, t602: f64, t1497: f64, t644: f64, t1469: f64, t606: f64, t30: f64, t33: f64, t70: f64, t2255: f64, zeta_threshold: f64, t36: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t4173 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk894(t1466, t602);
        let (t4178, t4181) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk895(t1497, t644, t1469, t606);
        let (t4182, t4186) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk896(t30, t33, t4181, t70, t2255, zeta_threshold);
        let t4187 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk897(t36, t4186);
    (t4173, t4178, t4181, t4182, t4186, t4187)
}
