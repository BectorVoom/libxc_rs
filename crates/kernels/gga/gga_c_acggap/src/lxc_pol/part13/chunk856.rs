//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 856/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk856<F: Float>(t1181: F, t15758: F, t31567: F, t599: F, t1089: F, t2079: F, t30052: F, t368: F, t31539: F, t7457: F, t7458: F, t7310: F, t7386: F, t7637: F, t7753: F, t7447: F, t7816: F) -> (F, F, F, F, F, F) {
    let t31593 = t31567 * t1181 * t599 * t15758;
    let t31597 = t2079 * t1089 * t368 * t30052;
    let t31598 = 0.10718504529517434243e-3 * t31597;
    let t31601 = t7457 * t7458 * t368 * t31539;
    let t31602 = 0.21437009059034868486e-3 * t31601;
    let t31603 = t7310 * t7386;
    let t31605 = t7637 * t7753;
    let t31607 = t7447 * t7816;
    (t31593, t31598, t31602, t31603, t31605, t31607)
}
