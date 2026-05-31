//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3257/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3257<F: Float>(t49834: F, t60183: F, t60213: F, t60558: F, t1518: F, t670: F, t10259: F, t116: F, t117: F, t13232: F, t13240: F, t13244: F, t13247: F, t13514: F, t1459: F, t1461: F, t18190: F, t18204: F, t18207: F, t18208: F, t18211: F, t18214: F, t1916: F, t1918: F, t2327: F, t2371: F, t4158: F, t4162: F, t4165: F, t4292: F, t49830: F, t572: F, t573: F, t5795: F, t5801: F, t5802: F, t5805: F, param_d: F) -> (F, F) {
    let t60560 = t49834 + t60183 + t60213 + t60558;
    let t60595 = t670 * t1518;
    let t60599 = F::cast_from(18.0_f64) * t5795 * t4162 + F::cast_from(18.0_f64) * t1916 * t13244 + F::cast_from(3.0_f64) * t1916 * t13247 + F::cast_from(3.0_f64) * t13232 * t1918 + F::cast_from(9.0_f64) * t5795 * t4165 + param_d * t60560 * t573 + F::cast_from(9.0_f64) * t18190 * t1461 + F::cast_from(18.0_f64) * t572 * t116 * t13514 * t670 + F::cast_from(18.0_f64) * t572 * t18207 * t2371 + F::cast_from(18.0_f64) * t1459 * t18204 + F::cast_from(36.0_f64) * t1459 * t18208 + F::cast_from(18.0_f64) * t1459 * t18211 + F::cast_from(18.0_f64) * t4158 * t5802 + F::cast_from(3.0_f64) * t572 * t117 * t49830 + F::cast_from(9.0_f64) * t4158 * t5805 + F::cast_from(6.0_f64) * t1916 * t13240 + F::cast_from(18.0_f64) * t572 * t2327 * t4292 + F::cast_from(6.0_f64) * t572 * t5801 * t10259 + F::cast_from(9.0_f64) * t1459 * t18214 + F::cast_from(18.0_f64) * t572 * t60595 * t2371;
    (t60560, t60599)
}
