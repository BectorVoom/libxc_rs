//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 958/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk958(t2331: f64, t946: f64, t251: f64, t2887: f64, t1075: f64, t237: f64, t240: f64) -> (f64, f64, f64) {
    let t9708 = t2331 * t946;
    let t9714 = t251 * t2887;
    let t9725 = t237 * t1075 * t240;
    (t9708, t9714, t9725)
}
