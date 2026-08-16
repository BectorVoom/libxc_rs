//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1205/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1205<F: Float>(t5612: F, t815: F, t6605: F, t1898: F, t5575: F, t249: F, t5628: F, t6621: F, t5619: F, t6614: F, t23048: F, t5587: F) -> (F, F, F, F, F, F, F) {
    let t28356 = t815 * t5612;
    let t28357 = t6605 * t28356;
    let t28359 = t5575 * t1898;
    let t28360 = t28359 * t249;
    let t28362 = t6621 * t5628;
    let t28364 = t6614 * t5619;
    let t28366 = t23048 * t5587;
    (t28356, t28357, t28359, t28360, t28362, t28364, t28366)
}
