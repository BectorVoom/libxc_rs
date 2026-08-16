//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta938 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3083;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta938<F: Float>(t1145: F, t141: F, t81207: F, t3417: F, t81169: F, t81173: F, t12254: F, t81165: F, t56176: F, t81439: F, t81442: F, t81445: F, t81448: F, t81451: F, t81454: F, t81457: F, t56183: F, t56236: F, t58536: F, t68389: F, t68399: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t44919: F, t52011: F, t77513: F) -> (F, F, F, F, F, F, F) {
        let (t81460, t81463, t81466, t81469, t81472) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3083::<F>(t1145, t141, t81207, t3417, t81169, t81173, t12254, t81165, t56176, t81439, t81442, t81445, t81448, t81451, t81454, t81457);
        let (t81485, t81489) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3084::<F>(t56183, t56236, t58536, t68389, t68399, t81224, t81228, t81230, t81232, t81234, t81236, t44919, t52011, t77513);
    (t81460, t81463, t81466, t81469, t81472, t81485, t81489)
}
