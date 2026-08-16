//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2256/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2256(t41142: f64, t41144: f64, t41149: f64, t41151: f64, t41155: f64, t41156: f64, t41158: f64, t41173: f64, t41181: f64, t41185: f64, t41187: f64, t12985: f64, t9577: f64) -> (f64, f64) {
    let t46759 = 0.49999999999999999998e-2_f64 * t41142 - 0.59999999999999999997e-1_f64 * t41144 - 0.15e-1_f64 * t41149 + 0.38888888888888888888e-2_f64 * t41151 + t41155 + 0.16851851851851851851e0_f64 * t41156 + 0.46666666666666666664e-1_f64 * t41158 + 0.1e-1_f64 * t41173 + 0.83333333333333333332e-3_f64 * t41181 - t41185 - 0.38888888888888888889e-1_f64 * t41187;
    let t46764 = t9577 * t12985;
    (t46759, t46764)
}
