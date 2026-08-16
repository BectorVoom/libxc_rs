//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 775/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk775<F: Float>(t524: F, t6511: F, t531: F, t2177: F, t2233: F, t1598: F, t489: F, t2225: F, t1541: F, t525: F, t146: F, t5052: F) -> (F, F, F, F, F, F, F, F) {
    let t6512 = t524 * t6511;
    let t6513 = t6512 * t531;
    let t6515 = t2177 * t2233;
    let t6517 = t1598 * t489;
    let t6518 = t524 * t6517;
    let t6519 = t6518 * t2225;
    let t6521 = t525 * t1541;
    let t6522 = t524 * t6521;
    let t6527 = t146 * t5052;
    (t6512, t6513, t6515, t6517, t6518, t6519, t6522, t6527)
}
