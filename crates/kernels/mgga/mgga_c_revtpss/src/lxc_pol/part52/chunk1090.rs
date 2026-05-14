//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1090/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1090<F: Float>(t127317: F, t128204: F, t128211: F, t128219: F, t128223: F, t128225: F, t128228: F, t128231: F, t2108: F, t25805: F, t28025: F, t28704: F, t28709: F, t32322: F, t33913: F, t6985: F, t7537: F, t7984: F, t8079: F, t8568: F) -> (F,) {
    let t128232 = t127317 * t2108 - 2.0 * t25805 * t7984 - 2.0 * t28025 * t7984 - 2.0 * t28704 * t6985 - t28709 * t8568 + 3.0 * t32322 * t8079 + t33913 * t7537 - t128204 - t128211 - t128219 + t128223 + t128225 - t128228 + t128231;
    (t128232,)
}
