//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 786/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk786(t1022: f64, t4818: f64, t3227: f64, t1092: f64, t1133: f64, t1767: f64) -> (f64, f64, f64, f64) {
    let t4819 = t1022 * t4818;
    let t4820 = t3227 * t4819;
    let t4821 = t1092 * t4820;
    let t4823 = t1767 * t1133;
    (t4819, t4820, t4821, t4823)
}
