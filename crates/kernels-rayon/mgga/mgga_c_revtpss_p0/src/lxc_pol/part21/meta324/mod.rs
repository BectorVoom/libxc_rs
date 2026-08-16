//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta324(t1100: f64, t3333: f64, t3335: f64, t389: f64, t2918: f64, t936: f64, t2874: f64, t2926: f64, t934: f64, t2924: f64, t1077: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11105, t11108, t11112, t11114, t11116, t11118, t11119, t11120, t11121) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1605(t1100, t3333, t3335, t389, t2918, t936, t2874, t2926, t934, t2924, t1077, t225);
    (t11105, t11108, t11112, t11114, t11116, t11118, t11119, t11120, t11121)
}
