//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1154;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta281<F: Float>(t207: F, t795: F, t9580: F, t2690: F, t841: F, t812: F, t849: F, t241: F, t6589: F, t67: F, t2632: F, t776: F, t815: F, t836: F, t2617: F, t2642: F, t1891: F, t246: F, t2628: F, t835: F, t831: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9583, t9601, t9602, t9607, t9627) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1154::<F>(t207, t795, t9580, t2690, t841, t812, t849, t241, t6589, t67, t2632, t776);
        let (t9638, t9642, t9645, t9646, t9667, t9671, t9672) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1155::<F>(t815, t836, t812, t2617, t2642, t1891, t67, t246, t2628, t835, t2690, t831);
    (t9583, t9601, t9602, t9607, t9627, t9638, t9642, t9645, t9646, t9667, t9671, t9672)
}
