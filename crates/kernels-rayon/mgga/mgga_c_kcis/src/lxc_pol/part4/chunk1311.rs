//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1311/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1311(t3728: f64, t5749: f64, t5753: f64, t11913: f64, t5645: f64, t16694: f64, t5662: f64, t4170: f64, t4160: f64, t1984: f64, t3245: f64, t12234: f64, t1943: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16804 = t3728 * t5749;
    let t16805 = 0.33163888888888888888e-2_f64 * t16804;
    let t16806 = t3728 * t5753;
    let t16808 = t11913 * t5645;
    let t16809 = 0.22109259259259259258e-2_f64 * t16808;
    let t16810 = t5662 * t16694;
    let t16811 = t4170 * t16810;
    let t16812 = t4160 * t16811;
    let t16820 = t3245 * t1984;
    let t16823 = t1943 * t12234;
    (t16804, t16805, t16806, t16808, t16809, t16812, t16820, t16823)
}
