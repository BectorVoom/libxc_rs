//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 796/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk796<F: Float>(t24873: F, t2876: F, t10703: F, t2883: F, t15312: F, t1882: F, t6367: F, t6376: F, t1495: F, t848: F) -> (F, F, F, F, F, F, F) {
    let t24874 = t24873 * t2876;
    let t24875 = t10703 * t24874;
    let t24878 = t24873 * t2883;
    let t24879 = t15312 * t24878;
    let t24882 = t1882 * t6367;
    let t24884 = t1882 * t6376;
    let t24886 = t848 * t1495;
    (t24874, t24875, t24878, t24879, t24882, t24884, t24886)
}
