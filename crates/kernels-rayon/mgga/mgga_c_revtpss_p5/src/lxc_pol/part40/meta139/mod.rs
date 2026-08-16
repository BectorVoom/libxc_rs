//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk665;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta139(t1065: f64, t999: f64, t906: f64, t1042: f64, t2868: f64, t2871: f64, t2878: f64, t2921: f64, t2929: f64, t3019: f64, t3021: f64, t3024: f64, t3028: f64, t3032: f64, t3036: f64, t1045: f64, t373: f64, t1031: f64, t196: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t3129, t3130, t3133) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk665(t1065, t999, t906, t1042, t2868, t2871, t2878, t2921, t2929, t3019, t3021, t3024, t3028, t3032, t3036);
        let (t3135, t3136, t3140) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk666(t1045, t3133, t373, t1042, t1031, t196);
    (t3129, t3130, t3133, t3135, t3136, t3140)
}
