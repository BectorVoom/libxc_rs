//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2628;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2629;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2630;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta667<F: Float>(t1248: F, t3604: F, t6688: F, t3720: F, t20266: F, t5312: F, t17475: F, t20293: F, t20318: F, t5308: F, t20310: F, t20306: F, t1260: F, t6601: F, t1222: F, t1266: F, t12784: F, t12855: F, t17437: F, t5304: F, t5309: F, t5313: F, t5373: F, t5391: F, t6640: F, t1264: F, t20272: F, t247: F, t5405: F, t6429: F, t3626: F, t6425: F, t1794: F, t5245: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t21119, t21120, t21121, t21126, t21129, t21134, t21137, t21140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2628::<F>(t1248, t3604, t6688, t3720, t20266, t5312, t17475, t20293, t20318, t5308, t20310, t20306);
        let (t21143, t21146) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2629::<F>(t1260, t6601, t1222, t1266, t12784, t12855, t17437, t21121, t21126, t21129, t21134, t21137, t21140, t5304, t5309, t5313, t5373, t5391, t6640);
        let (t21153, t21156, t21157, t21160, t21161, t21164) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2630::<F>(t1264, t20272, t247, t5405, t6429, t3626, t6425, t1794, t5245);
    (t21119, t21120, t21121, t21143, t21146, t21153, t21156, t21157, t21160, t21161, t21164)
}
