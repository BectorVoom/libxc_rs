//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 752/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk752<F: Float>(t1081: F, t2752: F, t2685: F, t3357: F, t3360: F, t1: F, t277: F, t9060: F, t2546: F, t3328: F, t2210: F, t2767: F, t3045: F, t7294: F, t3120: F, t3363: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9885 = t1081 * t2752;
    let t9887 = t3357 * t2685;
    let t9889 = t3360 * t2685;
    let t9894 = t277 * t1;
    let t9895 = t9894 * t9060;
    let t9896 = t2546 * t3328;
    let t9897 = t2210 * t9896;
    let t9898 = t9895 * t9897;
    let t9901 = t7294 * t3045 * t2767;
    let t9903 = t3363 * t3120;
    (t9885, t9887, t9889, t9894, t9895, t9896, t9897, t9898, t9901, t9903)
}
