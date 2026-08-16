//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1309/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1309(t16782: f64, t5653: f64, t4170: f64, t16771: f64, t4992: f64, t5659: f64, t86: f64, t5662: f64, t11913: f64, t5668: f64, t2038: f64, t3797: f64) -> (f64, f64, f64, f64) {
    let t16783 = t5653 * t16782;
    let t16784 = t4170 * t16783;
    let t16785 = t16771 * t16784;
    let t16788 = t86 * t4992 * t5659;
    let t16789 = t5662 * t16782;
    let t16790 = t4170 * t16789;
    let t16791 = t16788 * t16790;
    let t16793 = t11913 * t5668;
    let t16795 = t2038 * t3797;
    (t16785, t16791, t16793, t16795)
}
