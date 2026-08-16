//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1164/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1164(t3474: f64, t5043: f64, t1804: f64, t3361: f64, t375: f64, t3477: f64, t5068: f64, t14058: f64, t355: f64, t381: f64, t389: f64, t1180: f64, t5165: f64) -> (f64, f64, f64, f64, f64) {
    let t14751 = t3474 * t5043;
    let t14753 = t3361 * t1804;
    let t14754 = t375 * t14753;
    let t14756 = t3477 * t5068;
    let t14758 = t14058 * t355;
    let t14759 = t14758 * t381;
    let t14760 = t14759 * t389;
    let t14762 = t5165 * t1180;
    (t14751, t14754, t14756, t14760, t14762)
}
