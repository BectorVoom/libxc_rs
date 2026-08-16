//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1851;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta497(t5: f64, t30: f64, t265: f64, t393: f64, t26798: f64, t117: f64, t2126: f64, t2327: f64, t25743: f64, t2129: f64, t2258: f64, t25459: f64, t45: f64, t606: f64, t7594: f64, t2138: f64, t3650: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t2139: f64, t3655: f64, t1256: f64, t7610: f64, t3670: f64, t3666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26799, t26800, t26804, t26809, t26816, t26817) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1851(t5, t30, t265, t393, t26798, t117, t2126, t2327, t25743, t2129, t2258, t25459, t45, t606, t7594, t2138, t3650, dens_threshold, rho0, zeta_threshold);
        let (t26821, t26822, t26824, t26827) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1852(t2139, t3655, t1256, t7610, t2138, t3670, t3666);
    (t26799, t26800, t26804, t26809, t26816, t26817, t26821, t26822, t26824, t26827)
}
