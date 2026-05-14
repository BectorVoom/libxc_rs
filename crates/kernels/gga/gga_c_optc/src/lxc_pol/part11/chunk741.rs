//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 741/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk741<F: Float>(t2007: F, t4620: F, t4661: F, t654: F, t4665: F, t4668: F, t669: F, t106: F, t1281: F, t4675: F, t6976: F, t2144: F, t4699: F, t2182: F, t4703: F, t4706: F, t7018: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13262 = t2007 * t4620;
    let t13277 = t654 * t4661;
    let t13279 = t654 * t4665;
    let t13300 = t4668 * t669;
    let t13307 = t106 * t1281;
    let t13316 = t6976 * t4675;
    let t13330 = t2144 * t4699;
    let t13364 = t2182 * t4665;
    let t13366 = t2182 * t4661;
    let t13368 = t2144 * t4703;
    let t13373 = t7018 * t4706;
    (t13262, t13277, t13279, t13300, t13307, t13316, t13330, t13364, t13366, t13368, t13373)
}
