//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 932/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk932<F: Float>(t11865: F, t11867: F, t11870: F, t11873: F, t11879: F, t11890: F, t11893: F, t11895: F, t11898: F, t11900: F, t11911: F, t11914: F, t11919: F, t11928: F, t11931: F, t11935: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12236 = 0.20240885416666666668e-4 * t11865;
    let t12237 = 0.10821235962619981449e-3 * t11867;
    let t12238 = 0.15387284965264388985e-8 * t11870;
    let t12239 = 0.33764099580923002116e-6 * t11873;
    let t12240 = 0.10110318318802209383e-5 * t11879;
    let t12243 = 0.31675337336021900771e-5 * t11890;
    let t12244 = 0.33764099580923002116e-6 * t11893;
    let t12245 = 0.33764099580923002116e-6 * t11895;
    let t12246 = 0.20010856351627032588e-7 * t11898;
    let t12247 = 0.20047434126173032506e-6 * t11900;
    let t12251 = 0.10551281119038438161e-7 * t11911;
    let t12252 = 0.21102562238076876322e-7 * t11914;
    let t12253 = 0.39291224566445086216e-8 * t11919;
    let t12255 = 0.48340581405567281269e-8 * t11928;
    let t12256 = 0.67528199161846004232e-6 * t11931;
    let t12257 = 0.6746961805555555556e-5 * t11935;
    (t12236, t12237, t12238, t12239, t12240, t12243, t12244, t12245, t12246, t12247, t12251, t12252, t12253, t12255, t12256, t12257)
}
