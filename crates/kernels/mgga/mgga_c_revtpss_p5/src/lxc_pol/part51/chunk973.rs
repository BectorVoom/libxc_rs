//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 973/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk973<F: Float>(t2322: F, t8457: F, t1937: F, t25805: F, t28025: F, t6985: F, t6993: F, t4254: F, t1936: F, t7221: F, t651: F, t670: F, t8557: F) -> (F, F, F, F, F, F, F, F) {
    let t32301 = t2322 * t8457;
    let t32303 = t25805 * t1937;
    let t32305 = t28025 * t1937;
    let t32307 = t6985 * t6993;
    let t32309 = t4254 * t8457;
    let t32311 = t7221 * t1936;
    let t32312 = t651 * t32311;
    let t32316 = t8557 * t670;
    (t32301, t32303, t32305, t32307, t32309, t32311, t32312, t32316)
}
