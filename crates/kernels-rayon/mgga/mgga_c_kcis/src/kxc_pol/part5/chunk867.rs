//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 867/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk867(t303: f64, t7108: f64, t518: f64, t6281: f64, t1961: f64, t5792: f64, t6284: f64) -> (f64, f64, f64, f64, f64) {
    let t7109 = t303 * t7108;
    let t7113 = t518 * t6281;
    let t7116 = t5792 * t1961;
    let t7119 = t518 * t6284;
    let t7122 = t1961 * t1961;
    (t7109, t7113, t7116, t7119, t7122)
}
