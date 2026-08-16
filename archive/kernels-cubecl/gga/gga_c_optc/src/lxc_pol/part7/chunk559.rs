//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 559/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk559<F: Float>(t2661: F, t2750: F, t1: F, t2670: F, t297: F, t313: F, t2606: F, t312: F, t894: F, t937: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t2758 = t2661 * t2750;
    let t2760 = t2670 * t1 * t297;
    let t2761 = t313 * t2760;
    let t2764 = t312 * t2606;
    let t2765 = t2764 * t297;
    let t2766 = t894 * t2765;
    let t2769 = t937 * sigma0;
    (t2758, t2760, t2761, t2765, t2766, t2769)
}
