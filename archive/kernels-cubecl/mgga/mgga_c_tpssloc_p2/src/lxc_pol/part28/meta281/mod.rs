//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1176;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta281<F: Float>(t246: F, t9645: F, t232: F, t2379: F, t2628: F, t835: F, t812: F, t2635: F, t2690: F, t815: F, t831: F, t2617: F, t2638: F, t2639: F, t2681: F, t116: F, t126: F, t136: F, t16: F, t2386: F, t625: F, t2385: F, t686: F, t781: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9646, t9647, t9668, t9671, t9672, t9674) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1176::<F>(t246, t9645, t232, t2379, t2628, t835, t812, t2635, t2690, t815, t831, t2617, t2638);
        let (t9675, t9679, t9689, t9691, t9692, t9694) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1177::<F>(t831, t9674, t2639, t2681, t116, t126, t136, t16, t2386, t625, t2385, t686, t781);
    (t9646, t9647, t9668, t9671, t9672, t9674, t9675, t9679, t9689, t9691, t9692, t9694)
}
