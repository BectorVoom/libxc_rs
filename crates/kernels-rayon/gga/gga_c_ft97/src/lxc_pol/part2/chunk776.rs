//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 776/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk776(t11003: f64, t3621: f64, t3618: f64, t8675: f64, t3622: f64, t1068: f64, t8640: f64, t171: f64, t7741: f64, t11: f64, t41: f64, t3630: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12158 = t3621 * t11003;
    let t12162 = 4.0_f64 / 9.0_f64 * t8675 * t3618;
    let t12164 = 4.0_f64 / 9.0_f64 * t8675 * t3622;
    let t12165 = t8640 * t1068;
    let t12168 = 1.0_f64 / t171 / t7741;
    let t12169 = t11 * t12168;
    let t12170 = t41 * t12169;
    let t12171 = t12170 * t3630;
    (t12158, t12162, t12164, t12165, t12170, t12171)
}
