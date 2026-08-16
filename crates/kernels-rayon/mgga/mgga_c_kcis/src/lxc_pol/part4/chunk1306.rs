//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1306/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1306(t4134: f64, t531: f64, t1650: f64, t4136: f64, t4170: f64, t4160: f64, t1363: f64, t5623: f64, t1466: f64, t5869: f64, t1490: f64, t1464: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t16735 = t4134 * t531;
    let t16737 = t16735 * t1650 * t4136;
    let t16738 = t4170 * t16737;
    let t16739 = t4160 * t16738;
    let t16744 = t5623 * t1363;
    let t16751 = t5869 * t1466;
    let t16752 = t16751 * sigma2;
    let t16753 = t16752 * t1490;
    let t16754 = t1464 * t16753;
    (t16739, t16744, t16751, t16754)
}
