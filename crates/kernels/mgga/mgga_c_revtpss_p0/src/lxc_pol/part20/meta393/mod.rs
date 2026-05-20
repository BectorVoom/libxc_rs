//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1447;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta393<F: Float>(t141: F, t2908: F, t41325: F, t41310: F, t930: F, t41318: F, t9303: F, t931: F, t41308: F, t41312: F, t41320: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t41291: F, t41389: F, t41421: F, t964: F, t973: F, t981: F, t11591: F, t3026: F, t3034: F, t3030: F, t11465: F, t41225: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t41433, t41436, t41439, t41441, t41443) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1447::<F>(t141, t2908, t41325, t41310, t930, t41318, t9303, t931, t41308, t41312, t41320, t41327, t41330, t41332, t41334, t41336, t41365, t41367);
        let (t41445, t41449, t41451, t41453, t41455, t41459) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1448::<F>(t41291, t41389, t41421, t41443, t964, t973, t981, t11591, t3026, t3034, t3030, t11465, t41225);
    (t41433, t41436, t41439, t41441, t41445, t41449, t41451, t41453, t41455, t41459)
}
