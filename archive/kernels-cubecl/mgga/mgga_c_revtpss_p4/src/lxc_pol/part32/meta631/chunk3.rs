//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2046/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2046<F: Float>(t109118: F, t111066: F, t1310: F, t13426: F, t18227: F, t2014: F, t2055: F, t2093: F, t21658: F, t22483: F, t22506: F, t2322: F, t28737: F, t28760: F, t28939: F, t29506: F, t30138: F, t30558: F, t30563: F, t30581: F, t30589: F, t30617: F, t4248: F, t4254: F, t508: F, t5920: F, t651: F, t7235: F, t7374: F, t7474: F, t7488: F, t7489: F, t7536: F, t7898: F, t7978: F) -> F {
    let t111130 = -F::cast_from(2.0_f64) * t2322 * t30563 - F::cast_from(2.0_f64) * t4254 * t30563 - F::cast_from(2.0_f64) * t651 * t21658 * t2055 - t2014 * t7536 * t22483 + F::cast_from(2.0_f64) * t7235 * t30617 + F::cast_from(3.0_f64) * t2014 * t7488 * t109118 + t2093 * t22506 + F::cast_from(3.0_f64) * t29506 * t7489 - F::cast_from(4.0_f64) * t4248 * t28737 - F::cast_from(4.0_f64) * t30138 * t7374 - F::cast_from(4.0_f64) * t13426 * t7978 - F::cast_from(4.0_f64) * t18227 * t7978 - F::cast_from(4.0_f64) * t4248 * t28760 + F::cast_from(3.0_f64) * t7235 * t30581 - F::cast_from(2.0_f64) * t111066 * t508 - F::cast_from(2.0_f64) * t30589 * t1310 + F::cast_from(6.0_f64) * t7898 * t28939 - F::cast_from(2.0_f64) * t4254 * t30558 - F::cast_from(2.0_f64) * t651 * t7474 * t5920;
    t111130
}
