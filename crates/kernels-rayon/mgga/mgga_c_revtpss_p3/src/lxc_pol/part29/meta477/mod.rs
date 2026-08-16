//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta477(t27198: f64, t867: f64, t786: f64, t7063: f64, t14685: f64, t1941: f64, t14756: f64, t4435: f64, t7045: f64, t4426: f64, t7038: f64, t25245: f64, t4430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27212, t27213, t27216, t27221, t27222, t27224, t27226, t27228) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1753(t27198, t867, t786, t7063, t14685, t1941, t14756, t4435, t7045, t4426, t7038, t25245, t4430);
    (t27212, t27213, t27216, t27221, t27222, t27224, t27226, t27228)
}
