//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 826/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk826<F: Float>(t2660: F, t2767: F, t8639: F, t1081: F, t2807: F, t2752: F, t2685: F, t3357: F, t3360: F, t1: F, t277: F, t9060: F) -> (F, F, F, F, F, F, F) {
    let t9881 = t2660 * t8639 * t2767;
    let t9883 = t1081 * t2807;
    let t9885 = t1081 * t2752;
    let t9887 = t3357 * t2685;
    let t9889 = t3360 * t2685;
    let t9894 = t277 * t1;
    let t9895 = t9894 * t9060;
    (t9881, t9883, t9885, t9887, t9889, t9894, t9895)
}
