//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta285(t2435: f64, t2448: f64, t2440: f64, t887: f64, t2439: f64, t866: f64, t225: f64, t2461: f64, t2471: f64, t788: f64, t9288: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11000, t11003, t11004, t11006, t11007, t11008, t11013, t11015, t11017) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1510(t2435, t2448, t2440, t887, t2439, t866, t225, t2461, t2471, t788, t9288, t787);
    (t11000, t11003, t11004, t11006, t11007, t11008, t11013, t11015, t11017)
}
