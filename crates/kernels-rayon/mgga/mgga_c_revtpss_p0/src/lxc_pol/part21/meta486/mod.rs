//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta486(t11528: f64, t4595: f64, t11294: f64, t4636: f64, t4632: f64, t934: f64, t2874: f64, t1610: f64, t2918: f64, t2875: f64, t4635: f64, t11299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15377, t15379, t15380, t15382, t15383, t15385, t15386, t15388) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2068(t11528, t4595, t11294, t4636, t4632, t934, t2874, t1610, t2918, t2875, t4635, t11299);
    (t15377, t15379, t15380, t15382, t15383, t15385, t15386, t15388)
}
