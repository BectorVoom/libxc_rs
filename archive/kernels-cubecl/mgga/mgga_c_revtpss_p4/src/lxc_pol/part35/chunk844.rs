//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 844/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk844<F: Float>(t3801: F, t6748: F, t1209: F, t6695: F, t460: F, t487: F, t6564: F, t1770: F, t1811: F, t3172: F, t6618: F, t3711: F) -> (F, F, F, F, F, F) {
    let t20692 = t6748 * t3801;
    let t20697 = t1209 * t6695;
    let t20700 = t460 * t6695;
    let t20753 = t6564 * t487;
    let t20756 = t1770 * t1811;
    let t20783 = t3172 * t6618;
    let t20784 = t3711 * t20783;
    (t20692, t20697, t20700, t20753, t20756, t20784)
}
