//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 877/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk877<F: Float>(t1518: F, t7683: F, t1453: F, t1519: F, t2322: F, t27060: F, t28062: F, t28065: F, t28069: F, t28165: F, t28170: F, t28175: F, t28179: F, t29427: F, t29437: F, t4254: F, t569: F, t651: F, t671: F, t8158: F, t8237: F) -> (F, F) {
    let t29444 = t7683 * t1518;
    let t29451 = t1453 * t8237 - 2.0 * t1519 * t27060 - 2.0 * t2322 * t8158 - 2.0 * t29427 * t671 + t29437 * t569 - 2.0 * t29444 * t651 - 2.0 * t4254 * t8158 - t28062 - t28065 - t28069 + t28165 + t28170 + t28175 + t28179;
    (t29444, t29451)
}
