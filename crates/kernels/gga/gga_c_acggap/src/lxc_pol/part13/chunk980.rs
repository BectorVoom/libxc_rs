//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 980/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk980<F: Float>(t30993: F, t1165: F, t20138: F, t604: F, t7413: F, t1992: F, t30127: F, t7842: F, t8791: F, t1181: F, t33509: F, t599: F, t7346: F, t30262: F, t8406: F, t30268: F, t8903: F) -> (F, F, F, F, F, F) {
    let t35167 = 0.19055119163586549766e-2 * t30993;
    let t35172 = t7413 * t1165 * t604 * t20138;
    let t35176 = t30127 * t7842 * t1992 * t8791;
    let t35180 = t7346 * t1181 * t599 * t33509;
    let t35184 = t30262 * t7842 * t1992 * t8406;
    let t35186 = t30268 * t8903;
    (t35167, t35172, t35176, t35180, t35184, t35186)
}
