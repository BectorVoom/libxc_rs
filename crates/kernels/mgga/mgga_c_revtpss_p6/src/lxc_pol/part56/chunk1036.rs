//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1036/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1036<F: Float>(t122: F, t2466: F, t31780: F, t119928: F, t240: F, t822: F, t843: F, t31752: F, t31758: F, t119857: F, t1955: F, t136: F, t233: F, t2457: F) -> (F, F, F, F, F, F) {
    let t119930 = t31780 * t122 * t2466;
    let t119931 = t119928 * t119930;
    let t119934 = t822 * t843 * t240;
    let t119935 = t31752 * t119934;
    let t119936 = t119935 * t31758;
    let t119941 = t1955 * t119857;
    let t119955 = t233 * t136 * t2457;
    (t119930, t119931, t119935, t119936, t119941, t119955)
}
