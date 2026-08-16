//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta842 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3154;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta842<F: Float>(t1145: F, t141: F, t56232: F, t1729: F, t9303: F, t56153: F, t16894: F, t698: F, t16897: F, t16900: F, t2439: F, t5095: F, t12254: F, t56219: F, t3417: F, t56149: F, t43764: F, t56172: F, t43858: F, t43928: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t58151, t58153, t58156, t58158, t58160, t58162, t58165) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3154::<F>(t1145, t141, t56232, t1729, t9303, t56153, t16894, t698, t16897, t16900, t2439, t5095);
        let (t58168, t58171, t58174, t58177) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3155::<F>(t58165, t12254, t141, t56219, t3417, t56149, t43764, t56172, t43858, t43928, t58151, t58153, t58156, t58158, t58160, t58162);
    (t58151, t58153, t58156, t58158, t58160, t58162, t58165, t58168, t58171, t58174, t58177)
}
