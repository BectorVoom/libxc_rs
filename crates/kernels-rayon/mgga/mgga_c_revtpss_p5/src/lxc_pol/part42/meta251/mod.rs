//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk951;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk952;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk953;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta251(t1390: f64, t6844: f64, t828: f64, t124: f64, t6836: f64, t800: f64, t1414: f64, t6816: f64, t1882: f64, t4003: f64, t1868: f64, t543: f64, t5674: f64, t3936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6846, t6849, t6850, t6856, t6861) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk951(t1390, t6844, t828, t124, t6836, t800, t1414, t6816, t1882);
        let t6862 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk952(t4003, t6861);
        let (t6864, t6869) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk953(t1390, t6862, t828, t1868, t543);
        let (t6871, t6874) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk954(t5674, t6869, t3936, t543, t6861);
    (t6846, t6849, t6850, t6856, t6861, t6862, t6864, t6869, t6871, t6874)
}
