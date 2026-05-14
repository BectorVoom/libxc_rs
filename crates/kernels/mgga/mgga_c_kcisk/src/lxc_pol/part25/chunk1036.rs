//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1036/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1036<F: Float>(t2063: F, t5509: F, t12284: F, t1775: F, t2630: F, t5477: F, t2634: F, t2633: F, t5464: F, t15930: F, t5486: F, t17182: F, t7633: F, t2013: F, t5507: F, t7638: F) -> (F, F, F, F, F, F, F, F) {
    let t18401 = t2063 * t5509;
    let t18402 = t12284 * t18401;
    let t18403 = t1775 * t18402;
    let t18406 = t2630 * t5477;
    let t18408 = t2634 * t5477;
    let t18410 = t5464 * t2633;
    let t18413 = t5486 * t15930;
    let t18414 = t1775 * t18413;
    let t18421 = t17182 * t7633;
    let t18423 = 0.35981577432354634426e-1 * t2013 * t18421;
    let t18426 = t5507 * t7638;
    (t18401, t18403, t18406, t18408, t18410, t18414, t18423, t18426)
}
