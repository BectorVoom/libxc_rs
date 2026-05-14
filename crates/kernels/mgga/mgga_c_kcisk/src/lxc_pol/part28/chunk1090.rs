//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1090/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1090<F: Float>(t2012: F, t23768: F, t2014: F, t22591: F, t1775: F, t2009: F, t9208: F, t9189: F, t9184: F, t2633: F, t7574: F, t2004: F, t9183: F, t18406: F, t18408: F, t2013: F, t2016: F, t5471: F, t7581: F, t7615: F, t7634: F, t788: F, t9169: F) -> (F,) {
    let t24967 = t2012 * t23768;
    let t24972 = t2014 * t22591;
    let t24973 = t1775 * t24972;
    let t24976 = t9208 * t2009;
    let t24978 = t9189 * t2009;
    let t24980 = t9184 * t2009;
    let t24982 = t7574 * t2633;
    let t24985 = t2004 * t9183;
    let t24988 = -0.35981577432354634426e-1 * t7581 * t7615 - 0.11993859144118211475e-1 * t18406 + 0.31983624384315230601e-1 * t18408 + 0.10794473229706390328e0 * t7581 * t7634 + 0.87954967056866884153e-1 * t24967 * t2016 + 0.89953943580886586067e-2 * t5471 * t9169 + 0.89953943580886586067e-2 * t2013 * t24973 + 0.89953943580886586067e-2 * t24976 - 0.47975436576472845903e-1 * t24978 + 0.87954967056866884153e-1 * t24980 - 0.14392630972941853771e0 * t24982 * t788 + 0.26386490117060065246e0 * t24985 * t788;
    (t24988,)
}
