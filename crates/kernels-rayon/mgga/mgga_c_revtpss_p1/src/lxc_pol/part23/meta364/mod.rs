//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta364(t15191: f64, t4628: f64, t698: f64, t15127: f64, t15125: f64, t11452: f64, t1621: f64, t3014: f64, t4707: f64, t11509: f64, t1633: f64, t15168: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15192, t15197, t15198, t15209, t15210, t15211, t15241, t15258, t15266, t15301, t15312) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1681(t15191, t4628, t698, t15127, t15125, t11452, t1621, t3014, t4707, t11509, t1633, t15168);
    (t15192, t15197, t15198, t15209, t15210, t15211, t15241, t15258, t15266, t15301, t15312)
}
