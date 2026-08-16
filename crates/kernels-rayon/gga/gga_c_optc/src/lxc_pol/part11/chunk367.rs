//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 367/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk367(t1767: f64, t1770: f64, t1773: f64, t1777: f64, t1779: f64, t1782: f64) -> f64 {
    let t1784 = -0.42198333333333333333e0_f64 * t1767 + 0.84396666666666666666e0_f64 * t1770 + 0.39862222222222222223e0_f64 * t1773 + 0.68258333333333333333e-1_f64 * t1777 + 0.13651666666666666667e0_f64 * t1779 + 0.13692777777777777778e0_f64 * t1782;
    t1784
}
