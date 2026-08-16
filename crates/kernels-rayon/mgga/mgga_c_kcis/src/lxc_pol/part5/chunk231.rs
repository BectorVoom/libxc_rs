//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 231/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk231(t728: f64, t794: f64, t11: f64, t122: f64, t144: f64, t145: f64, t148: f64, t745: f64, t784: f64, t788: f64, t791: f64, t85: f64) -> (f64, f64) {
    let t795 = t794 * t728;
    let t804 = 0.619125e-2_f64 * t784 * t145 - 0.123825e-1_f64 * t788 * t791 - 0.619125e-2_f64 * t144 * t795 - 0.53062222222222222221e-1_f64 * t85 * t11 * t122 - 0.79593333333333333331e-1_f64 * t85 * t148 * t745;
    (t795, t804)
}
