//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 888/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk888<F: Float>(t42433: F, t4820: F, t6824: F, t12891: F, t1580: F, t1445: F, t3085: F, t597: F, t7995: F, t11392: F, t3159: F, t10348: F, t10485: F) -> (F, F, F, F, F) {
    let t42435 = t6824 * t4820 * t42433;
    let t42438 = F::cast_from(0.43710935587469654631e2_f64) * t1580 * t12891;
    let t42442 = F::cast_from(0.43710935587469654631e2_f64) * t597 * t1445 * t7995 * t3085;
    let t42444 = F::cast_from(0.25025342966295298669e1_f64) * t3159 * t11392;
    let t42448 = t10485 * t10348;
    (t42435, t42438, t42442, t42444, t42448)
}
