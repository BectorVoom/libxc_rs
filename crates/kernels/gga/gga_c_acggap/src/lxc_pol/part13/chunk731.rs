//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 731/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk731<F: Float>(t1095: F, t1479: F, t7476: F, t1980: F, t1988: F, t2304: F, t1089: F, t2302: F, t3201: F, t598: F, t137: F, t1487: F, t1083: F, t355: F, t506: F, t7458: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8555 = t7476 * t1095 * t1479;
    let t8556 = t1980 * t8555;
    let t8558 = t1988 * t2304;
    let t8561 = t1089 * t3201 * t2302;
    let t8562 = t598 * t8561;
    let t8564 = t137 * t1487;
    let t8566 = t1089 * t1083 * t8564;
    let t8567 = t598 * t8566;
    let t8569 = t355 * t506;
    let t8571 = t7458 * t1083 * t8569;
    (t8555, t8556, t8558, t8561, t8562, t8564, t8566, t8567, t8569, t8571)
}
