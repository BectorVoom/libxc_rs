//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2073;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta598(t10073: f64, t1444: f64, t2029: f64, t25929: f64, t26041: f64, t9664: f64, t2030: f64, t47567: f64, t26069: f64, t94806: f64, t1426: f64, t94609: f64, t7063: f64, t7286: f64, t7289: f64, t94810: f64, t26054: f64, t9686: f64, t25877: f64, t94801: f64, t25881: f64, t1419: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94857, t94865, t94867, t94876, t94878) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2073(t10073, t1444, t2029, t25929, t26041, t9664, t2030, t47567, t26069, t94806, t1426, t94609);
        let (t94880, t94882, t94884, t94886, t94887, t94889) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2074(t7063, t94878, t7286, t7289, t94810, t26054, t9686, t25877, t94801, t25881, t1419, t786);
    (t94857, t94865, t94867, t94876, t94878, t94880, t94882, t94884, t94886, t94887, t94889)
}
