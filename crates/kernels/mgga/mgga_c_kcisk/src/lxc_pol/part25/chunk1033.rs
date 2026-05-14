//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1033/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1033<F: Float>(t5014: F, t5507: F, t2642: F, t695: F, t5492: F, t5515: F, t7632: F, t7261: F, t12254: F, t5509: F, t10879: F, t2637: F, t2013: F, t2020: F, t7638: F, t1636: F) -> (F, F, F, F, F, F, F) {
    let t18338 = t5014 * t5507;
    let t18339 = t2642 * t695;
    let t18340 = t18339 * t5492;
    let t18341 = t18338 * t18340;
    let t18344 = t7632 * t5515;
    let t18345 = t7261 * t18344;
    let t18348 = t12254 * t2642;
    let t18349 = t18348 * t5509;
    let t18350 = t7261 * t18349;
    let t18355 = t10879 * t2637;
    let t18356 = t2013 * t18355;
    let t18358 = t2020 * t7638;
    let t18359 = t18358 * t1636;
    (t18339, t18340, t18341, t18345, t18350, t18356, t18359)
}
