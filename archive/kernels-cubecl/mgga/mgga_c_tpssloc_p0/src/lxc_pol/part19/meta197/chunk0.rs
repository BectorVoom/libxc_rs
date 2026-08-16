//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 863/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk863<F: Float>(t10250: F, t4518: F, t2775: F, t343: F, t2244: F, t2988: F, t2987: F, t3014: F, t2990: F, t2262: F, t972: F) -> (F, F, F, F, F, F, F) {
    let t10251 = t4518 * t10250;
    let t10254 = t343 * t2775;
    let t10255 = t10254 * t2244;
    let t10256 = t2988 * t10255;
    let t10259 = t2987 * t3014;
    let t10260 = t10259 * t2990;
    let t10263 = t2262 * t972;
    (t10251, t10254, t10255, t10256, t10259, t10260, t10263)
}
