//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1225/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1225<F: Float>(t20118: F, t20147: F, t3: F, t1851: F, t1858: F, t576: F, t6483: F, t112: F, t6470: F, t671: F, t1458: F, t4072: F) -> (F, F, F, F, F, F, F) {
    let t20148 = t20118 + t20147;
    let t20149 = t3 * t20148;
    let t20152 = t1851 * t1858;
    let t20158 = t576 * t6483;
    let t20162 = t6470 * t112;
    let t20173 = t576 * t671;
    let t20176 = t1458 * t4072;
    (t20148, t20149, t20152, t20158, t20162, t20173, t20176)
}
