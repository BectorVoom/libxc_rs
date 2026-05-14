//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 949/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk949<F: Float>(t1992: F, t30127: F, t7842: F, t8791: F, t1181: F, t33509: F, t599: F, t7346: F, t30262: F, t8406: F, t30268: F, t8903: F, t1165: F, t22040: F, t7351: F, t7493: F) -> (F, F, F, F, F) {
    let t35176 = t30127 * t7842 * t1992 * t8791;
    let t35180 = t7346 * t1181 * t599 * t33509;
    let t35184 = t30262 * t7842 * t1992 * t8406;
    let t35186 = t30268 * t8903;
    let t35190 = t7493 * t1165 * t7351 * t22040;
    (t35176, t35180, t35184, t35186, t35190)
}
