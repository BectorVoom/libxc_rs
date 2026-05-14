//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1108/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1108<F: Float>(t2606: F, t2640: F, t2644: F, t7467: F, t7471: F, t7488: F, t7906: F, t861: F, t24: F, t7920: F, t862: F, t7925: F, t1781: F, t866: F, t2548: F, t7256: F) -> (F, F, F, F, F, F, F) {
    let t25137 = t2640 * t7467 * t2606 * t2644;
    let t25145 = t7488 * t7471;
    let t25158 = t7906 * t861;
    let t25166 = t862 * t24 * t7920;
    let t25169 = t862 * t24 * t7925;
    let t25172 = t862 * t1781 * t866;
    let t25174 = t2548 * t7256;
    (t25137, t25145, t25158, t25166, t25169, t25172, t25174)
}
