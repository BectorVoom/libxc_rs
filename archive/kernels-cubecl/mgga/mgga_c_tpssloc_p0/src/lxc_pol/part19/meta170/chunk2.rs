//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 803/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk803<F: Float>(t2632: F, t776: F, t2645: F, t9626: F, t2678: F, t2646: F, t4180: F, t815: F, t836: F, t812: F) -> (F, F, F, F, F, F) {
    let t9627 = t2632 * t776;
    let t9629 = t2645 * t9626 * t9627;
    let t9632 = t2632 * t2678;
    let t9634 = t4180 * t2646 * t9632;
    let t9637 = t815 * t836;
    let t9638 = t812 * t9637;
    (t9627, t9629, t9632, t9634, t9637, t9638)
}
