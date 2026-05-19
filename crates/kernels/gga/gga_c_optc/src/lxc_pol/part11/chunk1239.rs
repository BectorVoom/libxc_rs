//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1239/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1239<F: Float>(t4675: F, t4723: F, t13190: F, t16380: F, t5: F, t56164: F, t116: F, t2024: F, t2034: F, t2113: F, t2159: F, t2168: F, t23098: F, t37600: F, t48744: F, t48748: F, t48750: F, t55933: F, t56170: F, t56174: F, t56193: F, t56209: F, t56252: F, t627: F, t675: F, t686: F, t6879: F, t696: F, t6993: F, t7002: F, t705: F, t9839: F) -> (F, F, F, F, F) {
    let t56373 = t4675 * t4675;
    let t56380 = t4723 * t4723;
    let t56391 = t16380 * t13190;
    let t56404 = t5 * t56164;
    let t56422 = -F::cast_from(0.17386322979577515709e0_f64) * t686 * t627 * t116 * t55933 + F::cast_from(0.72548214420044444092e0_f64) * t2168 * t2034 * t56391 - F::cast_from(0.30228422675018518372e-1_f64) * t705 * t56252 + F::cast_from(0.19184972257745086327e2_f64) * t37600 - F::cast_from(0.10882232163006666614e1_f64) * t6993 * t696 * t56174 - F::cast_from(0.14604511302845113195e2_f64) * t48744 - F::cast_from(0.8463958349005185144e1_f64) * t48748 + F::cast_from(0.2821319449668395048e0_f64) * t48750 - F::cast_from(0.31295381363239528276e1_f64) * t7002 * t675 * t56404 * t6879 + F::cast_from(0.12170426085704260996e1_f64) * t2113 * t675 * t56404 * t2024 + F::cast_from(0.13602790203758333267e0_f64) * t2159 * t696 * t56209 + F::cast_from(0.90685268025055555115e0_f64) * t23098 * t696 * t56170 - F::cast_from(0.5441116081503333307e1_f64) * t705 * t9839 * t56193;
    (t56373, t56380, t56391, t56404, t56422)
}
