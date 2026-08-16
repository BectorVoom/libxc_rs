//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2407/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2407(t13515: f64, t2837: f64, t2841: f64, t4351: f64, t2845: f64, t10697: f64, t4354: f64, t10701: f64, t1543: f64, t10705: f64, t1557: f64, t41618: f64) -> (f64, f64, f64, f64, f64) {
    let t49268 = 3.0_f64 * t13515 * t2837;
    let t49269 = t4351 * t2841;
    let t49271 = 0.48245938496077605201e2_f64 * t49269 * t2845;
    let t49273 = 1.0_f64 * t4354 * t10697;
    let t49274 = t1543 * t10701;
    let t49276 = 0.51726012919273400301e3_f64 * t49274 * t10705;
    let t49278 = 1.0_f64 * t41618 * t1557;
    (t49268, t49271, t49273, t49276, t49278)
}
