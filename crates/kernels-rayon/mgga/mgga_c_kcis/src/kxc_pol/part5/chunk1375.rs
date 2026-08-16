//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1375/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1375(t22640: f64, t4292: f64, t2062: f64, t6020: f64, t6016: f64, t6038: f64, t6044: f64, t21799: f64, t6011: f64, t17463: f64, t2061: f64, t5928: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22641 = t4292 * t22640;
    let t22643 = t6020 * t2062;
    let t22645 = t6016 * t6038;
    let t22647 = t6016 * t6044;
    let t22649 = t6011 * t21799;
    let t22650 = t17463 * t22649;
    let t22652 = t2061 * t5928;
    (t22641, t22643, t22645, t22647, t22650, t22652)
}
