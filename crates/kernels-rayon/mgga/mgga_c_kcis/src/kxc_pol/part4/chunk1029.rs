//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1029/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1029(t1341: f64, t3918: f64, t1559: f64, t4330: f64, t1563: f64, t4323: f64, t4355: f64, t3938: f64, t3947: f64, t11407: f64, t187: f64, t3910: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12755 = t1341 * t3918;
    let t12761 = t1559 * t4330;
    let t12767 = t4323 * t1563;
    let t12772 = t1559 * t4355;
    let t12780 = t3938 * t3947;
    let t12791 = 0.53272592592592592592e-1_f64 * t11407;
    let t12808 = t187 * t3910;
    (t12755, t12761, t12767, t12772, t12780, t12791, t12808)
}
