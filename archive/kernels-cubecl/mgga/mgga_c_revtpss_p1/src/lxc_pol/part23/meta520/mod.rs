//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2029;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2030;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta520<F: Float>(t20703: F, t247: F, t3719: F, t5357: F, t5373: F, t140: F, t6658: F, t1222: F, t6662: F, t1774: F, t5284: F, t1250: F, t3720: F, t1266: F, t17629: F, t21228: F, t21234: F, t21236: F, t21239: F, t21242: F, t3625: F, t3718: F, t5381: F, t5384: F, t5397: F, t20747: F, t369: F, t6593: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21246, t21249, t21251, t21252, t21254, t21255, t21257, t21258) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2029::<F>(t20703, t247, t3719, t5357, t5373, t140, t6658, t1222, t6662, t1774, t5284, t1250);
        let (t21259, t21264) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2030::<F>(t21258, t3720, t1222, t1266, t17629, t21228, t21234, t21236, t21239, t21242, t21246, t21249, t21252, t21255, t3625, t3718, t5381, t5384, t5397);
        let (t21267, t21271) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2031::<F>(t20747, t247, t3719, t369, t6593, t475);
    (t21246, t21249, t21251, t21252, t21254, t21255, t21257, t21258, t21259, t21264, t21267, t21271)
}
