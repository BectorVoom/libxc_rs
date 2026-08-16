//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1447;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta393(t141: f64, t2908: f64, t41325: f64, t41310: f64, t930: f64, t41318: f64, t9303: f64, t931: f64, t41308: f64, t41312: f64, t41320: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t41291: f64, t41389: f64, t41421: f64, t964: f64, t973: f64, t981: f64, t11591: f64, t3026: f64, t3034: f64, t3030: f64, t11465: f64, t41225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41433, t41436, t41439, t41441, t41443) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1447(t141, t2908, t41325, t41310, t930, t41318, t9303, t931, t41308, t41312, t41320, t41327, t41330, t41332, t41334, t41336, t41365, t41367);
        let (t41445, t41449, t41451, t41453, t41455, t41459) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1448(t41291, t41389, t41421, t41443, t964, t973, t981, t11591, t3026, t3034, t3030, t11465, t41225);
    (t41433, t41436, t41439, t41441, t41445, t41449, t41451, t41453, t41455, t41459)
}
