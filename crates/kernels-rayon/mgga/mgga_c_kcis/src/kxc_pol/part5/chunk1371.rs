//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1371/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1371(t21125: f64, t5968: f64, t17594: f64, t21130: f64, t21134: f64, t1392: f64, t1979: f64, t5441: f64, t3751: f64, t5427: f64, t21106: f64, t5976: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22582 = t5968 * t21125;
    let t22585 = t17594 * t21130;
    let t22588 = t5968 * t21134;
    let t22591 = t1392 * t1979;
    let t22592 = t22591 * t5441;
    let t22595 = t3751 * t1979;
    let t22596 = t22595 * t5427;
    let t22601 = t5976 * t21106;
    (t22582, t22585, t22588, t22592, t22596, t22601)
}
