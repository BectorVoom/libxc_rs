//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1480/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1480<F: Float>(t10743: F, t932: F, t2904: F, t938: F, t10524: F, t951: F, t10603: F, t10629: F, t315: F) -> (F, F, F, F, F) {
    let t10744 = t10743 * t932;
    let t10747 = t938 * t2904;
    let t10750 = t10524 * t951;
    let t10753 = t10603 * t951;
    let t10756 = t315 * t10629;
    (t10744, t10747, t10750, t10753, t10756)
}
