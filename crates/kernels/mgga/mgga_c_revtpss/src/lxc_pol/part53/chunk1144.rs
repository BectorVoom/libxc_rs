//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1144/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1144<F: Float>(t125606: F, t32705: F, t34230: F, t4075: F, t7063: F, t32240: F, t120976: F, t120982: F, t120987: F, t120994: F, t120997: F, t125570: F, t125573: F, t125576: F, t125578: F, t125580: F, t125582: F, t125584: F, t125590: F, t125594: F, t125596: F, t125599: F, t125603: F) -> (F, F) {
    let t125607 = t32705 * t125606;
    let t125609 = t34230 * t4075;
    let t125610 = t7063 * t125609;
    let t125611 = t125610 * t32240;
    let t125614 = F::cast_from(0.112937867033921868e-2_f64) * t125570 - F::cast_from(0.14874931683620404328e-2_f64) * t125573 - F::cast_from(0.14874931683620404328e-2_f64) * t125576 + F::cast_from(0.3718732920905101082e-3_f64) * t125578 + F::cast_from(0.3718732920905101082e-3_f64) * t125580 - F::cast_from(0.3718732920905101082e-4_f64) * t125582 + F::cast_from(0.66119071333692697238e-4_f64) * t125584 - F::cast_from(0.17354086964223805049e-2_f64) * t120976 - F::cast_from(0.14874931683620404328e-2_f64) * t125590 - F::cast_from(0.7437465841810202164e-4_f64) * t120982 - F::cast_from(0.3718732920905101082e-4_f64) * t120987 + F::cast_from(0.28912093960683998207e-1_f64) * t125594 - F::cast_from(0.51405703062096148813e-1_f64) * t125596 + F::cast_from(0.3718732920905101082e-3_f64) * t125599 - F::cast_from(0.7437465841810202164e-3_f64) * t125603 - F::cast_from(0.28559868832551176308e-1_f64) * t125607 - F::cast_from(0.14279934416275588154e-1_f64) * t125611 + F::cast_from(0.86770434821119025247e-3_f64) * t120994 + t120997;
    (t125609, t125614)
}
