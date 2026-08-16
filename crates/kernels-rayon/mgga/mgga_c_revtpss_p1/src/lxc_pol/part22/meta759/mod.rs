//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta759 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta759(t1041: f64, t1046: f64, t42994: f64, t1086: f64, t11213: f64, t3090: f64, t3057: f64, t3316: f64, t4891: f64, t3298: f64, t3059: f64, t3154: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t42996, t43038, t43043, t43044, t43049, t43050, t43051) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2839(t1041, t1046, t42994, t1086, t11213, t3090, t3057, t3316, t4891, t3298, t3059, t3154);
    (t42996, t43038, t43043, t43044, t43049, t43050, t43051)
}
