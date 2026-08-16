//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk971;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk972;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta259(t116: f64, t2198: f64, t670: f64, t117: f64, t8320: f64, t1459: f64, t1461: f64, t2207: f64, t2209: f64, t572: f64, t573: f64, t8336: f64, t1843: f64, t114: f64, t1513: f64, t8311: f64, t109: f64, t55: f64, t655: f64, t1509: f64, t8315: f64, t69: f64, t8258: f64, t8267: f64, t8310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t8342 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk971(t116, t2198);
        let (t8343, t8346, t8349, t8393) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk972(t670, t8342, t117, t8320, t1459, t1461, t2207, t2209, t572, t573, t8336, t1843, t2198);
        let (t8395, t8399, t8402, t8406) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk973(t114, t1513, t8311, t109, t55, t655, t1509, t8315, t69, t8258, t8267, t8310);
    (t8342, t8343, t8346, t8349, t8393, t8395, t8399, t8402, t8406)
}
