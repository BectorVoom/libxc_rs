//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1240/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1240<F: Float>(t1018: F, t1079: F, t11082: F, t11920: F, t11924: F, t11926: F, t1305: F, t1306: F, t1307: F, t1308: F, t2405: F, t330: F, t3381: F, t3643: F, t3645: F, t40767: F, t40869: F, t837: F, t838: F, t8420: F) -> F {
    let t40892 = (t40767 + t40869) * t330 + F::new(2.0) * t11920 * t837 * t330 + t3643 * t1305 * t330 + t3643 * t1307 * t330 + t11082 * t1018 * t330 + F::new(2.0) * t3381 * t2405 * t330 + F::new(2.0) * t11924 * t838 + t1079 * t8420 * t330 + F::new(2.0) * t11926 * t838 + t3645 * t1306 + t3645 * t1308;
    t40892
}
