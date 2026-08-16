//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1051/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1051(t3728: f64, t5749: f64, t5753: f64, t11913: f64, t5645: f64, t1984: f64, t3245: f64, t20: f64, t492: f64, t2194: f64, t1369: f64, t3999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16804 = t3728 * t5749;
    let t16805 = 0.33163888888888888888e-2_f64 * t16804;
    let t16806 = t3728 * t5753;
    let t16808 = t11913 * t5645;
    let t16809 = 0.22109259259259259258e-2_f64 * t16808;
    let t16820 = t3245 * t1984;
    let t16829 = t492 * t20;
    let t16830 = t16829 * t2194;
    let t16831 = t1369 * t3999;
    (t16804, t16805, t16806, t16808, t16809, t16820, t16830, t16831)
}
