//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 985/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk985<F: Float>(t19254: F, t446: F, t10758: F, t17749: F, t17753: F, t2857: F, t3281: F, t1091: F, t4129: F, t2665: F, t1212: F, t3746: F) -> (F, F, F, F, F, F) {
    let t19255 = t446 * t19254;
    let t19257 = t10758 * t17749;
    let t19258 = t446 * t19257;
    let t19260 = t2857 * t17753;
    let t19261 = t3281 * t19260;
    let t19263 = t1091 * t4129;
    let t19264 = t2665 * t19263;
    let t19265 = t446 * t19264;
    let t19267 = t3746 * t1212;
    (t19255, t19258, t19261, t19263, t19265, t19267)
}
