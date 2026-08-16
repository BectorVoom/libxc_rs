//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2072;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta628<F: Float>(t1235: F, t225: F, t461: F, t24574: F, t24626: F, t24617: F, t11553: F, t2121: F, t2123: F, t2122: F, t85628: F, t24884: F, t7288: F, t85660: F, t24758: F, t24637: F, t7294: F, t3427: F, t7295: F, t24901: F, t3640: F, t11947: F, t7394: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86415, t86424, t86426, t86451, t86452, t86456) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2072::<F>(t1235, t225, t461, t24574, t24626, t24617, t11553, t2121, t2123, t2122, t85628, t24884);
        let (t86473, t86475, t86494, t86501, t86513, t86517) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2073::<F>(t7288, t85660, t225, t24758, t24637, t7294, t2121, t3427, t7295, t24901, t3640, t11947, t7394);
    (t86415, t86424, t86426, t86451, t86452, t86456, t86473, t86475, t86494, t86501, t86513, t86517)
}
