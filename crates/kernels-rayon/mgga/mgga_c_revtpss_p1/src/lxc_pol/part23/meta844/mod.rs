//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta844 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2722;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta844(t1261: f64, t20981: f64, t3172: f64, t13033: f64, t21188: f64, t20985: f64, t20820: f64, t3704: f64, t17720: f64, t5381: f64, t20810: f64, t3711: f64, t17412: f64, t5378: f64, t17416: f64, t12915: f64, t20721: f64, t247: f64, t5384: f64, t21192: f64, t3647: f64, t21143: f64, t3636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70369, t70373, t70376, t70378, t70382, t70394) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2722(t1261, t20981, t3172, t13033, t21188, t20985, t20820, t3704, t17720, t5381, t20810, t3711);
        let (t70403, t70405, t70411, t70427, t70432) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2723(t17412, t5378, t17416, t5381, t12915, t20721, t247, t5384, t21192, t3647, t21143, t3636);
    (t70369, t70373, t70376, t70378, t70382, t70394, t70403, t70405, t70411, t70427, t70432)
}
