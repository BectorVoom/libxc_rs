//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1293/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1293<F: Float>(t20305: F, t626: F, t20308: F, t20343: F, t1858: F, t6470: F, t1851: F, t6483: F, t22453: F, t576: F, t112: F, t22430: F) -> (F, F, F, F, F, F, F) {
    let t75592 = t626 * t20305;
    let t75601 = t626 * t20308;
    let t75613 = t626 * t20343;
    let t75768 = t6470 * t1858;
    let t75774 = t1851 * t6483;
    let t75780 = t576 * t22453;
    let t75784 = t22430 * t112;
    (t75592, t75601, t75613, t75768, t75774, t75780, t75784)
}
