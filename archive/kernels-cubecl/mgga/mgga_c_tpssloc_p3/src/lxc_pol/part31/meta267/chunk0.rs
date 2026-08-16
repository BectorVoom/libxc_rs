//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1111/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1111<F: Float>(t1530: F, t25: F, t1597: F, t343: F, t1484: F, t28: F, t1458: F, t88: F) -> (F, F, F, F, F) {
    let t7545 = t25 * t1530;
    let t7577 = t1597 * t343;
    let t7649 = t28 * t1484;
    let t7656 = t28 * t1530;
    let t7676 = t88 * t1458;
    (t7545, t7577, t7649, t7656, t7676)
}
