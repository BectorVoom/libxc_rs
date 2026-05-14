//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1068/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1068<F: Float>(t11357: F, t26102: F, t11588: F, t27043: F, t35175: F, t3703: F, t11418: F, t3141: F, t34863: F, t505: F, t128: F, t567: F, t5741: F, t681: F, t35269: F, t35272: F, t35275: F, t35277: F, t35280: F, t35283: F) -> (F,) {
    let t35285 = t11357 * t26102;
    let t35287 = t11588 * t27043;
    let t35289 = t35175 * t3703;
    let t35293 = t11418 * t3141 * t34863 * t505;
    let t35298 = t11418 * t5741 * t681 * t128 * t567;
    let t35300 = 0.42206481990611010728e-7 * t35269 + 0.40022999988963401106e-7 * t35272 + 0.40096157891080460192e-6 * t35275 - 0.10258519928273509552e-8 * t35277 - 0.16908181191593721013e-5 * t35280 + 0.80192315782160920384e-6 * t35283 + 0.63309722985916516092e-7 * t35285 - 0.19336854506021130164e-7 * t35287 - 0.27041506680806477869e-6 * t35289 - 0.94685814672924837675e-4 * t35293 + 0.94685814672924837675e-4 * t35298;
    (t35300,)
}
