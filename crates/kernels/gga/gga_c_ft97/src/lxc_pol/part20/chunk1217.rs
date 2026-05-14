//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1217/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1217<F: Float>(t112812: F, t28772: F, t6317: F, t14075: F, t25037: F, t852: F, t9568: F, t13863: F, t99352: F, t25162: F, t28761: F, t2409: F, t7036: F, t24976: F, t25044: F, t4255: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113049 = t6317 * t28772 * t112812;
    let t113051 = t25037 * t14075;
    let t113053 = t6317 * t28772 * t113051;
    let t113055 = t9568 * t852;
    let t113056 = t99352 * t13863;
    let t113058 = t6317 * t113055 * t113056;
    let t113060 = t25162 * t28761;
    let t113061 = 2.0 / 9.0 * t113060;
    let t113062 = t7036 * t2409;
    let t113064 = t6317 * t24976 * t113062;
    let t113066 = t25044 * t4255;
    (t113049, t113051, t113053, t113056, t113058, t113060, t113061, t113062, t113064, t113066)
}
