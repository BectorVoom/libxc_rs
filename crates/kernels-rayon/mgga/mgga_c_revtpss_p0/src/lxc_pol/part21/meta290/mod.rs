//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1532;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1533;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta290(t10414: f64, t117: f64, t116: f64, t2319: f64, t10194: f64, t10259: f64, t1312: f64, t2322: f64, t2371: f64, t5523: f64, t670: f64, t2389: f64, t705: f64) -> (f64, f64, f64, f64) {
        let (t10415, t10416) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1532(t10414, t117, t116, t2319);
        let (t10426, t10428) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1533(t10194, t10259, t10415, t10416, t1312, t2322, t2371, t5523, t670, t2389, t705);
    (t10415, t10416, t10426, t10428)
}
