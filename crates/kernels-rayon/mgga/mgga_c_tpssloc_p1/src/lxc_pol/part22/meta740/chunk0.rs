//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2437/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2437(t13520: f64, t17507: f64, t13727: f64, t17510: f64, t10661: f64, t4395: f64, t5730: f64, t21303: f64, t42028: f64, t912: f64, t21300: f64, t2792: f64) -> (f64, f64, f64, f64, f64) {
    let t69335 = 18.0_f64 * t13520 * t17507;
    let t69337 = 12.0_f64 * t13727 * t17510;
    let t69340 = 0.28947563097646563121e3_f64 * t10661 * t5730 * t4395;
    let t69343 = 0.62071215503128080361e4_f64 * t42028 * t21303 * t912;
    let t69346 = 2.0_f64 * t2792 * t21300 * t912;
    (t69335, t69337, t69340, t69343, t69346)
}
