//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1065/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1065(t13467: f64, t2970: f64, t4904: f64, t743: f64, t4907: f64, t2635: f64, t4580: f64) -> (f64, f64, f64, f64) {
    let t13468 = t2970 * t13467;
    let t13472 = 0.4705225e-4_f64 * t743 * t4904;
    let t13473 = t743 * t4907;
    let t13475 = t4580 * t2635;
    (t13468, t13472, t13473, t13475)
}
