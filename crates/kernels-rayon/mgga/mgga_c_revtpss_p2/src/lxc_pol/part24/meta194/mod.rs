//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta194 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta194(t1386: f64, t2681: f64, t820: f64, t4000: f64, t843: f64, t136: f64, t4011: f64, t240: f64, t532: f64, t549: f64, t72: f64, t595: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9909, t9918, t9921, t9934, t9940, t9941, t9942, t9948) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk923(t1386, t2681, t820, t4000, t843, t136, t4011, t240, t532, t549, t72, t595, t66);
    (t9909, t9918, t9921, t9934, t9940, t9941, t9942, t9948)
}
