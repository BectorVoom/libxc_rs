//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2721/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2721<F: Float>(t50058: F, t40125: F, t40127: F, t40132: F, t2408: F, t775: F, t40139: F, t11075: F, t14318: F, t14436: F, t14468: F, t2403: F, t2430: F, t262: F, t40131: F, t40137: F, t4433: F, t4541: F) -> (F, F, F, F, F, F) {
    let t50059 = F::cast_from(24.0_f64) * t50058;
    let t50063 = F::cast_from(0.18311447306006545054e-3_f64) * t40125;
    let t50064 = F::cast_from(0.73245789224026180215e-3_f64) * t40127;
    let t50065 = F::cast_from(0.17544670867903938621e1_f64) * t40132;
    let t50066 = t2408 * t775;
    let t50070 = F::cast_from(12.0_f64) * t40139;
    let t50078 = F::cast_from(18.0_f64) * t14468 * t262 * t4541 * t775 + F::cast_from(18.0_f64) * t11075 * t4433 * t4541 + F::cast_from(18.0_f64) * t14318 * t2430 * t4541 + F::cast_from(18.0_f64) * t14436 * t2403 * t50066 - t40131 - t40137 + t50059 - t50063 + t50064 - t50065 + t50070;
    (t50059, t50063, t50064, t50065, t50070, t50078)
}
