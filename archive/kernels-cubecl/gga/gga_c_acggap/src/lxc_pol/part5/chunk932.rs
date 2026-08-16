//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 932/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk932<F: Float>(t314: F, t3644: F, t150: F, t383: F, t390: F, t1005: F, t3697: F, t12357: F, t174: F, t384: F, t386: F, t387: F) -> (F, F, F, F, F) {
    let t14401 = t3644 * t314;
    let t14402 = t14401 * t150;
    let t14405 = F::cast_from(0.85748036236139473944e-3_f64) * t14402 * t383 * t390;
    let t14414 = F::cast_from(0.85748036236139473944e-3_f64) * t1005 * t3697;
    let t14419 = F::cast_from(0.21437009059034868486e-3_f64) * t384 * t386 * t387 * t174 * t12357;
    (t14401, t14402, t14405, t14414, t14419)
}
