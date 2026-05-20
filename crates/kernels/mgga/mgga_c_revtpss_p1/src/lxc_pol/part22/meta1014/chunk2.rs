//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3493/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3493<F: Float>(t20050: F, t3188: F, t20054: F, t1063: F, t18946: F, t247: F, t3109: F, t11714: F, t11991: F, t20046: F, t3106: F, t42257: F, t42270: F, t42274: F, t53542: F, t53557: F, t53559: F, t6323: F, t6327: F, t6331: F) -> F {
    let t65801 = t3188 * t20050;
    let t65803 = t3188 * t20054;
    let t65807 = t1063 * t247 * t3109 * t18946;
    let t65819 = -t42257 / F::new(972.0) - F::cast_from(0.5081365110289746604e-3_f64) * t42270 - F::cast_from(0.1270341277572436651e-3_f64) * t42274 - t53542 / F::new(324.0) + F::cast_from(0.31758531939310916276e-3_f64) * t65801 + F::cast_from(0.19055119163586549765e-3_f64) * t65803 + F::cast_from(0.19055119163586549765e-3_f64) * t65807 - F::cast_from(0.2540682555144873302e-2_f64) * t11714 * t6327 - F::cast_from(0.15244095330869239812e-2_f64) * t11714 * t6323 - F::cast_from(0.15244095330869239812e-2_f64) * t3106 * t20046 - F::cast_from(0.28582678745379824648e-3_f64) * t11991 * t6331 + F::cast_from(0.17149607247227894789e-2_f64) * t53557 + F::cast_from(0.3811023832717309953e-3_f64) * t53559;
    t65819
}
