//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3232/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3232<F: Float>(t50113: F, t40150: F, t14341: F, t4311: F, t18253: F, t18268: F, t198: F, t2394: F, t2430: F, t262: F, t39989: F, t4541: F, t50080: F, t5966: F, t61265: F, t61269: F, t61274: F, t61283: F, t61286: F) -> (F, F, F, F) {
    let t61287 = F::cast_from(8.0_f64) * t50113;
    let t61288 = F::cast_from(2.0_f64) * t40150;
    let t61289 = t4311 * t14341;
    let t61290 = F::cast_from(16.0_f64) * t61289;
    let t61291 = F::cast_from(6.0_f64) * t198 * t2430 * t262 * t5966 - F::cast_from(6.0_f64) * t18268 * t2394 * t4541 + F::cast_from(24.0_f64) * t18253 * t50080 - t39989 + t61265 + t61269 + t61274 + t61283 + t61286 + t61287 + t61288 + t61290;
    (t61287, t61288, t61290, t61291)
}
