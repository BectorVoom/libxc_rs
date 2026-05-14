//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1235/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1235<F: Float>(t32555: F, t3274: F, t10340: F, t32552: F, t12814: F, t2692: F, t3186: F, t1009: F, t31997: F, t1053: F, t9337: F, t37234: F, t2935: F, t3063: F, t31896: F, t927: F) -> (F, F, F, F, F, F, F) {
    let t111201 = 3.0 * t32555 * t3274;
    let t111203 = 6.0 * t10340 * t32552;
    let t111206 = 2.0 * t3186 * t2692 * t12814;
    let t111219 = t31997 * t1009;
    let t111221 = 3.0 * t111219 * t1053;
    let t111223 = t9337 * t12814;
    let t111224 = t37234 * t2692;
    let t111228 = t31896 * t2935 * t3063 * t927;
    (t111201, t111203, t111206, t111221, t111223, t111224, t111228)
}
