//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2305/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2305<F: Float>(t1834: F, t5210: F, t1807: F, t5318: F, t1842: F, t5353: F, t3887: F, t1814: F, t5333: F, t1338: F, t6434: F, t1352: F) -> (F, F, F, F, F, F) {
    let t19635 = t5210 * t1834;
    let t19644 = t1807 * t5318;
    let t19647 = t1842 * t5353;
    let t19648 = t3887 * t19647;
    let t19654 = t1814 * t5333;
    let t19657 = t1338 * t6434;
    let t19658 = t19657 * t1352;
    (t19635, t19644, t19648, t19654, t19657, t19658)
}
