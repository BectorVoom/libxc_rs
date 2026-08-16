//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 202/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk202(t449: f64, t637: f64, t103: f64, t32: f64, t5: f64, t28: f64, t14: f64, t15: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t638 = t449 * t637;
    let t643 = 0.11073577833333333333e-2_f64 * t5 * t103 * t32;
    let t644 = t28 * t28;
    let t645 = 1.0_f64 / t644;
    let t646 = t14 * t645;
    let t647 = 1.0_f64 / t15;
    (t638, t643, t644, t645, t646, t647)
}
