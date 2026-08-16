//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2569/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2569<F: Float>(t1667: F, t9709: F, t14712: F, t699: F, t2403: F, t4778: F, t14750: F, t690: F) -> (F, F, F, F) {
    let t50846 = t9709 * t1667;
    let t50848 = t699 * t14712;
    let t50853 = t2403 * t4778;
    let t50903 = t690 * t14750;
    (t50846, t50848, t50853, t50903)
}
