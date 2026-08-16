//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2797/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2797(t16957: f64, t41011: f64, t16662: f64, t213: f64, t221: f64, t41142: f64, t41144: f64, t41149: f64, t41155: f64, t41156: f64, t41185: f64, t41187: f64, t41190: f64, t41192: f64, t41194: f64, t41197: f64, t4127: f64, t46764: f64, t46768: f64, t776: f64) -> (f64, f64) {
    let t59100 = t41011 * t16957;
    let t59134 = 0.16666666666666666666e-2_f64 * t41142 - 0.39999999999999999998e-1_f64 * t41144 - 0.49999999999999999998e-2_f64 * t41149 + t41155 + 0.11234567901234567901e0_f64 * t41156 - t41185 - 0.12962962962962962963e-1_f64 * t41187 + 0.6574074074074074074e-1_f64 * t41190 - 0.52777777777777777776e-2_f64 * t41192 + 0.38888888888888888889e-1_f64 * t41194 + 0.15833333333333333333e-1_f64 * t41197 - 0.39999999999999999998e-1_f64 * t46764 + 0.66666666666666666664e-2_f64 * t46768 + 0.99999999999999999996e-2_f64 * t4127 * t221 * t213 * t16662 * t776;
    (t59100, t59134)
}
