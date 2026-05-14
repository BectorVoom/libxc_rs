//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 966/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk966<F: Float>(t8392: F, t9850: F, t761: F, t9577: F, t10094: F, t9788: F, t2596: F, t8232: F, t1882: F, t9835: F, t2591: F, t3281: F, t726: F, t2614: F, t10137: F, t9855: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t42853 = t8392 * t9850;
    let t42859 = t761 * t9577;
    let t42864 = t8392 * t10094;
    let t42874 = t8392 * t9788;
    let t42879 = t8232 * t2596;
    let t42881 = t1882 * t9835;
    let t42884 = t8232 * t2591;
    let t42894 = t3281 * t726;
    let t42896 = t8232 * t2614;
    let t42898 = t1882 * t10137;
    let t42914 = t8392 * t9855;
    (t42853, t42859, t42864, t42874, t42879, t42881, t42884, t42894, t42896, t42898, t42914)
}
