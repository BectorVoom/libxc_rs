//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1106/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1106<F: Float>(t1992: F, t30262: F, t7842: F, t8406: F, t30268: F, t8903: F, t1165: F, t22040: F, t7351: F, t7493: F, t1181: F, t20311: F, t7426: F) -> (F, F, F, F) {
    let t35184 = t30262 * t7842 * t1992 * t8406;
    let t35186 = t30268 * t8903;
    let t35190 = t7493 * t1165 * t7351 * t22040;
    let t35191 = F::new(0.47172138434406228102e-2) * t35190;
    let t35194 = t7426 * t1181 * t7351 * t20311;
    (t35184, t35186, t35191, t35194)
}
