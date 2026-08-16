//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1950;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta524(t33: f64, t265: f64, t502: f64, t27754: f64, t1469: f64, t2003: f64, t27821: f64, t4186: f64, t57: f64, t606: f64, t7215: f64, t7877: f64, t27762: f64, t196: f64, t197: f64, t5528: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t2035: f64, t7313: f64, t7898: f64, t1032: f64, t1892: f64, t1955: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t27822, t27830, t27833) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1950(t33, t265, t502, t27754, t1469, t2003, t27821, t4186, t57, t606, t7215, t7877, t27762, t196, t197, t5528, dens_threshold, rho1, zeta_threshold);
        let (t27834, t27835, t27836, t27837) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1951(t2035, t27833, t7313, t7898, t1032, t1892, t1955);
    (t27822, t27830, t27833, t27834, t27835, t27836, t27837)
}
