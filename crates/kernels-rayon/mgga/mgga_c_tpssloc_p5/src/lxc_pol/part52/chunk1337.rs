//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1337/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1337(t1998: f64, t5187: f64, t59: f64, t6926: f64, t5287: f64, t6936: f64, t6943: f64, t22779: f64, t32714: f64, t5230: f64, t8465: f64, t8467: f64) -> (f64, f64, f64, f64) {
    let t120405 = t6926 * t1998 * t59 * t5187;
    let t120408 = t6936 * t6943 * t5287;
    let t120410 = t22779 * t32714;
    let t120413 = t5230 * t8465 * t8467;
    (t120405, t120408, t120410, t120413)
}
