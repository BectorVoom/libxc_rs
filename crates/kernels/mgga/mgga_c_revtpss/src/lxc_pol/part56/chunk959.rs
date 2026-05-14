//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 959/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk959<F: Float>(t120976: F, t120982: F, t120987: F, t120994: F, t120997: F, t125570: F, t125573: F, t125576: F, t125578: F, t125580: F, t125582: F, t125584: F, t125590: F, t125594: F, t125596: F, t125599: F, t125603: F, t125607: F, t125611: F) -> (F,) {
    let t125614 = 0.112937867033921868e-2 * t125570 - 0.14874931683620404328e-2 * t125573 - 0.14874931683620404328e-2 * t125576 + 0.3718732920905101082e-3 * t125578 + 0.3718732920905101082e-3 * t125580 - 0.3718732920905101082e-4 * t125582 + 0.66119071333692697238e-4 * t125584 - 0.17354086964223805049e-2 * t120976 - 0.14874931683620404328e-2 * t125590 - 0.7437465841810202164e-4 * t120982 - 0.3718732920905101082e-4 * t120987 + 0.28912093960683998207e-1 * t125594 - 0.51405703062096148813e-1 * t125596 + 0.3718732920905101082e-3 * t125599 - 0.7437465841810202164e-3 * t125603 - 0.28559868832551176308e-1 * t125607 - 0.14279934416275588154e-1 * t125611 + 0.86770434821119025247e-3 * t120994 + t120997;
    (t125614,)
}
