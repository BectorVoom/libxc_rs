//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1009/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1009<F: Float>(t7219: F, t7230: F, t5030: F, t8820: F, t1785: F, t7261: F, t695: F, t8814: F, t1060: F, t11179: F, t5015: F, t2364: F, t7268: F, t2464: F, t6714: F, t17337: F) -> (F, F, F, F, F, F) {
    let t23344 = t7219 * t7230;
    let t23346 = t5030 * t8820;
    let t23347 = t23346 * t1785;
    let t23348 = t7261 * t23347;
    let t23355 = t8814 * t695;
    let t23356 = t23355 * t1060;
    let t23357 = t11179 * t23356;
    let t23360 = t8820 * t695;
    let t23361 = t23360 * t1060;
    let t23362 = t5015 * t23361;
    let t23365 = t2364 * t7268;
    let t23366 = t5015 * t23365;
    let t23369 = t6714 * t2464;
    let t23370 = t17337 * t23369;
    (t23344, t23348, t23357, t23362, t23366, t23370)
}
