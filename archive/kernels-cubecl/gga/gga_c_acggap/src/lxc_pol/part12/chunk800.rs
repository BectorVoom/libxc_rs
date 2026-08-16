//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 800/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk800<F: Float>(t2095: F, t8505: F, t137: F, t1579: F, t336: F, t578: F, t1494: F, t2041: F, t1498: F, t355: F, t535: F, t5720: F, t599: F) -> (F, F, F, F, F, F, F, F) {
    let t8744 = t2095 * t8505;
    let t8747 = t336 * t1579 * t137;
    let t8748 = t578 * t8747;
    let t8754 = t2041 * t1494;
    let t8756 = t2041 * t1498;
    let t8771 = t535 * t355;
    let t8772 = t2095 * t8771;
    let t8774 = t599 * t5720;
    (t8744, t8747, t8748, t8754, t8756, t8771, t8772, t8774)
}
