//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1097/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1097<F: Float>(t11894: F, t833: F, t1013: F, t1074: F, t11060: F, t11066: F, t1292: F, t1295: F, t1300: F, t2394: F, t2400: F, t3370: F, t3633: F, t37020: F, t37023: F, t6693: F, t829: F, t8398: F, t8409: F, t8412: F, t8415: F) -> (F,) {
    let t40764 = t11894 * t833;
    let t40767 = -0.768e1 * t37020 * t2400 - 0.768e1 * t11066 * t8412 - 0.384e1 * t11066 * t8415 - 0.1536e2 * t37023 * t8409 - 0.128e1 * t1300 * t11060 * t1013 - 0.256e1 * t1300 * t3370 * t2394 - 0.128e1 * t1300 * t1074 * t8398 - 0.256e1 * t1300 * t11894 * t829 - 0.128e1 * t1300 * t3633 * t1292 - 0.384e1 * t6693 * t3633 * t1295 - 0.256e1 * t40764 * t829;
    (t40767,)
}
