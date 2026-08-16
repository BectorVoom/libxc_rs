//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta647(t11452: f64, t2962: f64, t41306: f64, t3335: f64, t1071: f64, t3043: f64, t12032: f64, t342: f64, t11902: f64, t378: f64, t3046: f64, t3259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41895, t41908, t41937, t41993, t42013, t42038, t42041, t42044) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2432(t11452, t2962, t41306, t3335, t1071, t3043, t12032, t342, t11902, t378, t3046, t3259);
    (t41895, t41908, t41937, t41993, t42013, t42038, t42041, t42044)
}
