//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1330/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1330<F: Float>(t252: F, t9584: F, t2591: F, t852: F, t225: F, t2711: F, t2594: F, t2690: F, t841: F, t812: F) -> (F, F, F, F, F, F) {
    let t9585 = t9584 * t252;
    let t9587 = t2591 * t852;
    let t9590 = t2711 * t225;
    let t9593 = t2594 * t225;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    (t9585, t9587, t9590, t9593, t9600, t9601)
}
