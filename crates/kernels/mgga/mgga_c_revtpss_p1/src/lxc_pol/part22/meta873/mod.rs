//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta873 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3035;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta873<F: Float>(t11044: F, t14983: F, t14485: F, t15014: F, t9303: F, t10510: F, t14987: F, t14991: F, t41066: F, t10982: F, t1568: F, t9646: F, t252: F, t2769: F, t2782: F, t4533: F, t886: F, t10995: F, t11049: F, t14990: F, t14986: F, t2453: F, t10506: F, t2458: F, t4470: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51231, t51233, t51237, t51239, t51241, t51246) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3035::<F>(t11044, t14983, t14485, t15014, t9303, t10510, t14987, t14991, t41066, t10982, t1568, t9646);
        let (t51251, t51256, t51258, t51259, t51262) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3036::<F>(t252, t2769, t2782, t4533, t886, t10995, t11049, t14990, t14986, t2453, t10506, t2458, t4470);
    (t51231, t51233, t51237, t51239, t51241, t51246, t51251, t51256, t51258, t51259, t51262)
}
