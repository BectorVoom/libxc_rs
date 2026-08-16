//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2749/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2749(t41279: f64, t5499: f64, t12945: f64, t4205: f64, t46208: f64, t4194: f64, t5398: f64, t607: f64, t750: f64, t46217: f64, t13130: f64, t32: f64, t5519: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57959 = 12.0_f64 * t41279 * t5499;
    let t57960 = t4205 * t12945;
    let t57961 = 8.0_f64 * t57960;
    let t57962 = 0.20508037716432813315e4_f64 * t46208;
    let t57965 = t4194 * t750 * t5398 * t607;
    let t57966 = 24.0_f64 * t57965;
    let t57970 = 16.0_f64 * t46217;
    let t57972 = 8.0_f64 * t4205 * t13130;
    let t57973 = t32 * t5519;
    (t57959, t57961, t57962, t57966, t57970, t57972, t57973)
}
