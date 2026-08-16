//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta379(t4930: f64, t994: f64, t1678: f64, t3046: f64, t3057: f64, t379: f64, t1078: f64, t1651: f64, t342: f64, t1071: f64, t1647: f64, t378: f64, t4743: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16302, t16305, t16312, t16313, t16333, t16340, t16362) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1719(t4930, t994, t1678, t3046, t3057, t379, t1078, t1651, t342, t1071, t1647, t378, t4743);
    (t16302, t16305, t16312, t16313, t16333, t16340, t16362)
}
