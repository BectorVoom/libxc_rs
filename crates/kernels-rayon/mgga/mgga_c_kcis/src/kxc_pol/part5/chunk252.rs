//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 252/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk252(t934: f64, t939: f64, t250: f64, t253: f64, t324: f64, t251: f64, t287: f64) -> (f64, f64, f64, f64) {
    let t940 = t939 * t934;
    let t943 = t250 * t324 * t253;
    let t944 = 0.82156666666666666667e-1_f64 * t943;
    let t945 = t251 * t287;
    (t940, t943, t944, t945)
}
