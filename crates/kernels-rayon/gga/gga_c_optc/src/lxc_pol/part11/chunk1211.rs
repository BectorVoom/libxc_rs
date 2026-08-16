//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1211/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1211(t43: f64, t13007: f64, t16231: f64, t1933: f64, t22014: f64, t3331: f64, t4565: f64, t55901: f64, t55906: f64, t55912: f64, t607: f64, t4570: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t55916 = piecewise3(t44, 0.0_f64, -56.0_f64 / 81.0_f64 * t22014 * t55901 + 16.0_f64 / 9.0_f64 * t13007 * t4565 - 2.0_f64 / 3.0_f64 * t1933 * t55906 - 8.0_f64 / 9.0_f64 * t3331 * t16231 + 2.0_f64 / 3.0_f64 * t607 * t55912);
    let t55917 = t4570 * t4570;
    (t55916, t55917)
}
