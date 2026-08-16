//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1230/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1230<F: Float>(t129467: F, t1937: F, t2126: F, t4292: F, t34446: F, t6993: F, t127336: F, t127340: F, t127341: F, t127346: F, t129455: F, t129457: F, t129459: F, t129461: F, t129463: F, t129465: F) -> (F, F) {
    let t129468 = t129467 * t1937;
    let t129470 = t2126 * t4292;
    let t129471 = t129470 * t1937;
    let t129473 = t34446 * t6993;
    let t129476 = -t129455 - F::cast_from(3.0_f64) * t127336 - t127340 - F::cast_from(2.0_f64) * t129457 - F::cast_from(2.0_f64) * t129459 - F::cast_from(2.0_f64) * t129461 - F::cast_from(2.0_f64) * t129463 - F::cast_from(2.0_f64) * t129465 - F::cast_from(2.0_f64) * t129468 - F::cast_from(2.0_f64) * t129471 - F::cast_from(2.0_f64) * t129473 + F::cast_from(3.0_f64) * t127341 - t127346;
    (t129470, t129476)
}
