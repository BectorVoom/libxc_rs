//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1075/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1075<F: Float>(t2925: F, t321: F, t1: F, t2021: F, t325: F, t8720: F, t2089: F, t107: F, t787: F, t24536: F, t7290: F, t21502: F) -> (F, F, F, F, F, F, F, F) {
    let t24884 = t321 * t2925;
    let t24885 = t24884 * t1;
    let t24886 = t2021 * t24885;
    let t24908 = t325 * t8720;
    let t24926 = t2089 * t8720;
    let t24945 = t787 * t24884 * t107;
    let t25055 = t7290 * t24536;
    let t25059 = t21502 * t24536;
    (t24884, t24885, t24886, t24908, t24926, t24945, t25055, t25059)
}
