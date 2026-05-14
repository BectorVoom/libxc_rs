//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 933/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk933<F: Float>(t11863: F, t11865: F, t11867: F, t11870: F, t11873: F, t11879: F, t11890: F, t11893: F, t11895: F, t11898: F, t11900: F, t11911: F, t11914: F, t11919: F, t11882: F, t11885: F, t11903: F, t11906: F, t11908: F) -> (F,) {
    let t12235 = 0.20240885416666666668e-4 * t11863;
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
    let t12254 = -t12235 - t12236 + t12237 + t12238 + t12239 + t12240 - 0.90579542097823505425e-7 * t11882 - 0.52838066223730378165e-7 * t11885 + t12243 - t12244 - t12245 - t12246 - t12247 + 0.90579542097823505425e-7 * t11903 - 0.18115908419564701085e-6 * t11906 + 0.18115908419564701085e-6 * t11908 - t12251 - t12252 + t12253;
    (t12254,)
}
