//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1589/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1589<F: Float>(t23967: F, t6492: F, t2031: F, t22550: F, t6495: F, t7032: F, t7025: F, t9231: F, t6486: F, t240: F, t67: F, t1864: F) -> (F, F, F, F, F, F, F) {
    let t23968 = t23967 * t6492;
    let t23970 = t2031 * t22550;
    let t23973 = t6495 * t7032;
    let t23975 = t9231 * t7025;
    let t23978 = t6486 * t7032;
    let t23992 = t240 * t67;
    let t23993 = t23992 * t1864;
    (t23968, t23970, t23973, t23975, t23978, t23992, t23993)
}
