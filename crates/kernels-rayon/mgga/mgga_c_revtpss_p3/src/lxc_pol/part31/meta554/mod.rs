//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta554(t1955: f64, t6888: f64, t225: f64, t30055: f64, t2022: f64, t6861: f64, t4003: f64, t26079: f64, t543: f64, t7301: f64, t6843: f64, t1882: f64, t7910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30071, t30074, t30081, t30082, t30088, t30089, t30095, t30096, t30100) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1960(t1955, t6888, t225, t30055, t2022, t6861, t4003, t26079, t543, t7301, t6843, t1882, t7910);
    (t30071, t30074, t30081, t30082, t30088, t30089, t30095, t30096, t30100)
}
