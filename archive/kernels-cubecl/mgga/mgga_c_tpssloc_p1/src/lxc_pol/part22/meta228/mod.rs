//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta228 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1289;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1290;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta228<F: Float>(t2588: F, t9577: F, t21: F, t59: F, t207: F, t795: F, t2690: F, t841: F, t812: F, t849: F, t241: F, t6589: F, t67: F, t2632: F, t776: F, t815: F, t836: F, t2617: F, t2642: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9579, t9580, t9583, t9600, t9601) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1289::<F>(t2588, t9577, t21, t59, t207, t795, t2690, t841, t812);
        let (t9602, t9607, t9627, t9637, t9638) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1290::<F>(t849, t9601, t241, t6589, t67, t2632, t776, t815, t836, t812);
        let t9642 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1291::<F>(t2617, t2642);
    (t9579, t9580, t9583, t9600, t9601, t9602, t9607, t9627, t9637, t9638, t9642)
}
