//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1915;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta468(t19872: f64, t3092: f64, t1062: f64, t15670: f64, t247: f64, t3109: f64, t6096: f64, t1063: f64, t11672: f64, t11774: f64, t15796: f64, t15829: f64, t19858: f64, t19861: f64, t19864: f64, t19867: f64, t19869: f64, t3091: f64, t375: f64, t4839: f64, t6268: f64) -> (f64, f64, f64, f64, f64) {
        let (t19873, t19878) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1915(t19872, t3092, t1062, t15670);
        let (t19882, t19883, t19885) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1916(t247, t3109, t6096, t1063, t11672, t11774, t15796, t15829, t19858, t19861, t19864, t19867, t19869, t19873, t19878, t3091, t375, t4839, t6268);
    (t19873, t19878, t19882, t19883, t19885)
}
