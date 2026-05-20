//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1477;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta485<F: Float>(t3670: F, t6594: F, t3718: F, t44546: F, t6689: F, t3717: F, t70994: F, t3617: F, t6587: F, t3147: F, t6593: F, t3594: F, t3597: F, t1244: F, t17628: F, t5373: F, t3655: F, t6595: F, t1222: F, t6658: F, t697: F, t6662: F, t1209: F, t1284: F, t6695: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t71280, t71294, t71513, t71543, t71691, t71693) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1477::<F>(t3670, t6594, t3718, t44546, t6689, t3717, t70994, t3617, t6587, t3147, t6593, t3594, t3597);
        let (t71699, t71718, t71744, t71928, t71931, t72267) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1478::<F>(t1244, t3594, t71691, t17628, t5373, t3655, t6595, t1222, t6658, t697, t6662, t1209, t1284, t6695);
    (t71280, t71294, t71513, t71543, t71693, t71699, t71718, t71744, t71928, t71931, t72267)
}
