//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1230/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1230<F: Float>(t12511: F, t17023: F, t17026: F, t1745: F, t20471: F, t20568: F, t20571: F, t20573: F, t20576: F, t20579: F, t20582: F, t20597: F, t3447: F, t435: F, t5120: F, t5125: F, t5143: F, t6487: F, t6503: F) -> (F,) {
    let t20602 = 2.0 * t17026 * t1745 + 2.0 * t5120 * t5143 - 2.0 * t12511 * t6487 + 1.0 * t3447 * t6503 + t20471 - 0.19751673498613801407e-1 * t20568 + t20571 - t20573 - t20576 + t20579 + t20582 - 0.310907e-1 * t20597 * t435 - 4.0 * t17023 * t5125;
    (t20602,)
}
