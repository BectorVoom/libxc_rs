//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 894/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk894<F: Float>(t535: F, t7457: F, t7458: F, t7459: F, t1089: F, t3201: F, t598: F, t8484: F, t8489: F, t1980: F, t525: F, t922: F, t1181: F, t30282: F, t599: F, t30090: F, t8952: F) -> (F, F, F, F, F, F, F) {
    let t33894 = t7457 * t7458 * t535 * t7459;
    let t33898 = t598 * t1089 * t3201 * t8484;
    let t33901 = t3201 * t8489;
    let t33903 = t1980 * t7458 * t33901;
    let t33911 = t525 * t922;
    let t33914 = t30282 * t1181 * t599 * t33911;
    let t33916 = t30090 * t8952;
    (t33894, t33898, t33901, t33903, t33911, t33914, t33916)
}
