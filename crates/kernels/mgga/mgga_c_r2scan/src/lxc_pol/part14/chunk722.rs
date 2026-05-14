//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 722/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk722<F: Float>(t551: F, t6499: F, t2196: F, t481: F, t6343: F, t566: F, t560: F, t549: F, t110: F, t6238: F, t524: F, t531: F, t2177: F, t2233: F, t1598: F, t489: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6500 = t551 * t6499;
    let t6501 = t2196 * t6500;
    let t6503 = t6343 * t481;
    let t6504 = t551 * t6503;
    let t6505 = t566 * t6504;
    let t6507 = t6343 * t560;
    let t6508 = t551 * t6507;
    let t6509 = t549 * t6508;
    let t6511 = t6238 * t110;
    let t6512 = t524 * t6511;
    let t6513 = t6512 * t531;
    let t6515 = t2177 * t2233;
    let t6517 = t1598 * t489;
    (t6501, t6503, t6505, t6507, t6509, t6511, t6512, t6513, t6515, t6517)
}
