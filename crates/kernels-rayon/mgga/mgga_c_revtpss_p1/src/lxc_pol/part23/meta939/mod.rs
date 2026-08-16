//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta939 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3085;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta939(t24274: f64, t698: f64, t52011: f64, t58027: f64, t77513: f64, t24271: f64, t1134: f64, t6449: f64, t16851: f64, t16854: f64, t43888: f64, t58153: f64, t58165: f64, t58543: f64, t81242: f64, t81245: f64, t81489: f64, t24317: f64, t43821: f64, t20356: f64, t5079: f64, t24312: f64, t3390: f64, t16857: f64, t20337: f64, t5071: f64, t43946: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81491, t81494, t81496, t81499, t81501, t81506) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3085(t24274, t698, t52011, t58027, t77513, t24271, t1134, t6449, t16851, t16854, t43888, t58153, t58165, t58543, t81242, t81245, t81489);
        let (t81509, t81511, t81514, t81516, t81518, t81521) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3086(t1134, t24317, t43821, t20356, t5079, t24312, t3390, t16857, t6449, t20337, t5071, t43946);
    (t81491, t81494, t81496, t81499, t81501, t81506, t81509, t81511, t81514, t81516, t81518, t81521)
}
