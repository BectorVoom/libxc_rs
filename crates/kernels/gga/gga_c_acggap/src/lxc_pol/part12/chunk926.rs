//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 926/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk926<F: Float>(t30934: F, t8450: F, t30937: F, t8597: F, t8602: F, t1165: F, t4718: F, t7351: F, t7426: F, t1181: F, t4818: F, t599: F, t8463: F, t30543: F, t8469: F, t4521: F) -> (F, F, F, F, F, F, F) {
    let t34618 = t30934 * t8450;
    let t34620 = t30937 * t8597;
    let t34622 = t30937 * t8602;
    let t34626 = t7426 * t1165 * t7351 * t4718;
    let t34630 = t8463 * t1181 * t599 * t4818;
    let t34632 = t30543 * t8469;
    let t34636 = t7426 * t1165 * t7351 * t4521;
    (t34618, t34620, t34622, t34626, t34630, t34632, t34636)
}
