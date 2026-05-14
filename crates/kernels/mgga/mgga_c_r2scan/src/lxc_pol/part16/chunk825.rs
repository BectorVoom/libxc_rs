//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 825/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk825<F: Float>(t2924: F, t818: F, t2928: F, t826: F, t1010: F, t2391: F, t2938: F, t1217: F, t2358: F, t2368: F, t313: F, t6678: F, t9598: F, t9608: F, t9613: F, t9623: F, t9631: F, t9635: F) -> (F, F, F, F, F) {
    let t9640 = t2924 * t818;
    let t9650 = t2928 * t826;
    let t9653 = t1010 * t2391;
    let t9657 = t2938 * t826;
    let t9673 = 3.0 / 10.0 * t313 * (-10.0 / 27.0 * t9598 + 20.0 / 9.0 * t2358 * t1217 + 10.0 / 9.0 * t9608 + 5.0 / 3.0 * t9613 - 10.0 / 27.0 * t9623 - 20.0 / 9.0 * t2368 * t1217 + 10.0 / 9.0 * t9631 + 5.0 / 3.0 * t9635) - t6678;
    (t9640, t9650, t9653, t9657, t9673)
}
