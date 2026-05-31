//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3677/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3677<F: Float>(t1160: F, t20597: F, t16688: F, t16840: F, t5068: F, t58339: F, t5109: F, t58466: F, t16652: F, t17092: F, t16662: F, t12243: F, t20574: F) -> (F, F, F, F, F, F, F) {
    let t69565 = t20597 * t1160;
    let t69569 = F::cast_from(12.0_f64) * t16840 * t16688;
    let t69571 = F::cast_from(8.0_f64) * t58339 * t5068;
    let t69573 = F::cast_from(0.64327917994770140268e2_f64) * t58466 * t5109;
    let t69575 = F::cast_from(8.0_f64) * t17092 * t16652;
    let t69577 = F::cast_from(0.64327917994770140268e2_f64) * t16840 * t16662;
    let t69579 = F::cast_from(12.0_f64) * t12243 * t20574;
    (t69565, t69569, t69571, t69573, t69575, t69577, t69579)
}
