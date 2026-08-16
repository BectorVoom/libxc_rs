//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1202/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1202<F: Float>(t23384: F, t6692: F, t1049: F, t6688: F, t6691: F, t1054: F, t1065: F, t1921: F, t986: F, t2978: F, t344: F, t381: F) -> (F, F, F, F, F, F, F, F) {
    let t23579 = t23384 * t6692;
    let t23581 = t6688 * t1049;
    let t23582 = t23581 * t6691;
    let t23587 = t1054 * t1065;
    let t23588 = t1921 * t23587;
    let t23589 = t986 * t23588;
    let t23592 = t2978 * t344;
    let t23593 = t23592 * t381;
    (t23579, t23581, t23582, t23587, t23588, t23589, t23592, t23593)
}
