//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2062;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta484(t3006: f64, t4711: f64, t11509: f64, t1633: f64, t2988: f64, t4670: f64, t953: f64, t1622: f64, t2962: f64, t2944: f64, t4673: f64, t2970: f64, t4669: f64, t1634: f64, t15127: f64, t15168: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15163: f64, t15166: f64, t15170: f64, t15173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15263, t15266, t15267, t15274, t15277, t15280, t15283) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2062(t3006, t4711, t11509, t1633, t2988, t4670, t953, t1622, t2962, t2944, t4673, t2970, t4669);
        let (t15284, t15287, t15290, t15301, t15315) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2063(t15283, t953, t1622, t2944, t1634, t2988, t15127, t15168, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15170, t15173);
    (t15263, t15266, t15267, t15274, t15277, t15280, t15283, t15284, t15287, t15290, t15301, t15315)
}
