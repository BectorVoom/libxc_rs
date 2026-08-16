//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 488/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk488<F: Float>(t1: F, t2925: F, t106: F, t316: F, t795: F, t313: F, t1022: F, t701: F, t739: F, t8502: F, t2610: F, t7290: F) -> (F, F, F, F, F, F, F, F) {
    let t8632 = t2925 * t1;
    let t8633 = t8632 * t106;
    let t8634 = t8633 * t316;
    let t8637 = t795 * t2925;
    let t8638 = t313 * t8637;
    let t8669 = t1022 * t701;
    let t8670 = t739 * t8669;
    let t8682 = t739 * t8502;
    let t8756 = t2610 * t8669;
    let t8769 = t7290 * t8502;
    (t8634, t8637, t8638, t8669, t8670, t8682, t8756, t8769)
}
