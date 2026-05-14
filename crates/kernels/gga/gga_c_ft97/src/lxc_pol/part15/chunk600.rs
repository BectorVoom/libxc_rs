//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 600/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk600<F: Float>(t4545: F, t487: F, t1882: F, t4617: F, t4574: F, t4565: F, t4561: F, t4557: F, t15606: F, t15609: F, t15612: F, t15891: F, t15894: F, t15899: F, t1775: F, t4515: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16246 = t4545 * t487;
    let t16255 = t1882 * t4617;
    let t16296 = t1882 * t4574;
    let t16298 = t1882 * t4565;
    let t16300 = t1882 * t4561;
    let t16302 = t1882 * t4557;
    let t16336 = 2.0 / 27.0 * t15606;
    let t16337 = 2.0 / 9.0 * t15609;
    let t16338 = t15612 / 9.0;
    let t16342 = t15891 / 3.0;
    let t16343 = 2.0 / 3.0 * t15894;
    let t16346 = 2.0 / 9.0 * t15899;
    let t16373 = t1775 * t4515;
    (t16246, t16255, t16296, t16298, t16300, t16302, t16336, t16337, t16338, t16342, t16343, t16346, t16373)
}
