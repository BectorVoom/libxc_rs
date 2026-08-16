//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1193/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1193(t13691: f64, t13694: f64, t16209: f64, t16211: f64, t16213: f64, t21669: f64, t21671: f64, t21675: f64, t21679: f64, t21681: f64, t21684: f64, t21687: f64, t21691: f64, t21695: f64) -> f64 {
    let t21700 = 0.5868e1_f64 * t21669 - 0.3912e1_f64 * t21671 - 0.22005e1_f64 * t21675 + 0.1467e1_f64 * t21679 - 0.1956e1_f64 * t21681 + 0.1467e1_f64 * t21684 + 0.7335e0_f64 * t21687 + 0.8802e1_f64 * t21691 - 0.22005e1_f64 * t21695 + 0.978e0_f64 * t16209 - 0.4564e1_f64 * t16211 + 0.76066666666666666667e1_f64 * t16213 - t13691 + t13694;
    t21700
}
