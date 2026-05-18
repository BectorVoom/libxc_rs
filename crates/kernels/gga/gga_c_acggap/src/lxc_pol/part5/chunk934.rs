//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 934/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk934<F: Float>(t1255: F, t980: F, t1252: F, t3646: F, t457: F, t13485: F, t13487: F, t452: F, t1004: F, t3829: F, t1035: F, t14255: F) -> (F, F, F, F, F, F) {
    let t14460 = t980 * t1255;
    let t14478 = t980 * t1252;
    let t14480 = t3646 * t457;
    let t14485 = F::new(0.15805078039045227836e2) * t13485 * t452 * t13487;
    let t14486 = t1004 * t3829;
    let t14490 = F::new(0.39512695097613069591e1) * t1035 * t452 * t14255;
    (t14460, t14478, t14480, t14485, t14486, t14490)
}
