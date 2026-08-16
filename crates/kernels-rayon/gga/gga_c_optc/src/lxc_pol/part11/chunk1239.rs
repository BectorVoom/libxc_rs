//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1239/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1239(t4675: f64, t4723: f64, t13190: f64, t16380: f64, t5: f64, t56164: f64, t116: f64, t2024: f64, t2034: f64, t2113: f64, t2159: f64, t2168: f64, t23098: f64, t37600: f64, t48744: f64, t48748: f64, t48750: f64, t55933: f64, t56170: f64, t56174: f64, t56193: f64, t56209: f64, t56252: f64, t627: f64, t675: f64, t686: f64, t6879: f64, t696: f64, t6993: f64, t7002: f64, t705: f64, t9839: f64) -> (f64, f64, f64, f64, f64) {
    let t56373 = t4675 * t4675;
    let t56380 = t4723 * t4723;
    let t56391 = t16380 * t13190;
    let t56404 = t5 * t56164;
    let t56422 = -0.17386322979577515709e0_f64 * t686 * t627 * t116 * t55933 + 0.72548214420044444092e0_f64 * t2168 * t2034 * t56391 - 0.30228422675018518372e-1_f64 * t705 * t56252 + 0.19184972257745086327e2_f64 * t37600 - 0.10882232163006666614e1_f64 * t6993 * t696 * t56174 - 0.14604511302845113195e2_f64 * t48744 - 0.8463958349005185144e1_f64 * t48748 + 0.2821319449668395048e0_f64 * t48750 - 0.31295381363239528276e1_f64 * t7002 * t675 * t56404 * t6879 + 0.12170426085704260996e1_f64 * t2113 * t675 * t56404 * t2024 + 0.13602790203758333267e0_f64 * t2159 * t696 * t56209 + 0.90685268025055555115e0_f64 * t23098 * t696 * t56170 - 0.5441116081503333307e1_f64 * t705 * t9839 * t56193;
    (t56373, t56380, t56391, t56404, t56422)
}
