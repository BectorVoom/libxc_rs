//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1493/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1493<F: Float>(t118276: F, t118413: F, t118456: F, t118500: F, t670: F, t8362: F, t116: F, t31555: F, t117758: F, t1459: F, t1518: F, t1916: F, t1918: F, t2187: F, t22559: F, t22568: F, t31340: F, t31365: F, t31610: F, t31613: F, t31616: F, t35739: F, t4292: F, t572: F, t573: F, t5795: F, t5805: F, t6941: F, t6948: F, t8289: F, t8299: F, t8377: F, t8383: F, t8386: F, param_d: F) -> (F, F) {
    let t118502 = t118276 + t118413 + t118456 + t118500;
    let t118507 = t670 * t8362;
    let t118527 = t116 * t31555;
    let t118533 = F::cast_from(12.0_f64) * t117758 * t1518 * t572 + t118502 * t573 * param_d + F::cast_from(12.0_f64) * t118507 * t1518 * t572 + F::cast_from(6.0_f64) * t118527 * t572 * t670 + F::cast_from(12.0_f64) * t35739 * t4292 * t572 + F::cast_from(12.0_f64) * t1459 * t31610 + F::cast_from(6.0_f64) * t1459 * t31613 + F::cast_from(3.0_f64) * t1459 * t31616 + F::cast_from(12.0_f64) * t1916 * t31365 + F::cast_from(6.0_f64) * t1918 * t31340 + F::cast_from(12.0_f64) * t2187 * t22559 + F::cast_from(3.0_f64) * t2187 * t22568 + F::cast_from(12.0_f64) * t5795 * t8383 + F::cast_from(6.0_f64) * t5795 * t8386 + F::cast_from(6.0_f64) * t5805 * t8377 + F::cast_from(3.0_f64) * t6941 * t8299 + F::cast_from(3.0_f64) * t6948 * t8289;
    (t118502, t118533)
}
