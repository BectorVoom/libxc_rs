//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 531/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk531<F: Float>(t6275: F, t8392: F, t312: F, t6260: F, t1483: F, t8232: F, t1882: F, t6284: F, t6293: F, t2680: F, t6308: F, t6310: F, t681: F, t2781: F, t683: F) -> (F, F, F, F, F, F, F, F) {
    let t24903 = t8392 * t6275;
    let t24908 = t312 * t6260;
    let t24955 = 4.0 / 27.0 * t8232 * t1483;
    let t24960 = t1882 * t6284;
    let t24962 = t1882 * t6293;
    let t24964 = t2680 * t6260;
    let t24974 = t6308 * t681 * t6310;
    let t24976 = t683 * t2781;
    (t24903, t24908, t24955, t24960, t24962, t24964, t24974, t24976)
}
