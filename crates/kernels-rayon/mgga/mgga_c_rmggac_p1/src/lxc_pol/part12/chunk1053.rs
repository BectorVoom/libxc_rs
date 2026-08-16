//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1053/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1053(t118: f64, t128: f64, t1986: f64, t1994: f64, t5735: f64, t30137: f64, t681: f64, t2034: f64, t30174: f64, t2310: f64, t7944: f64, t2191: f64, t8597: f64) -> (f64, f64, f64, f64, f64) {
    let t41846 = t1994 * t1986 * t118 * t128 * t5735;
    let t41848 = t30137 * t681;
    let t41850 = t30174 * t2034;
    let t41863 = t7944 * t2310;
    let t41865 = t2191 * t8597;
    (t41846, t41848, t41850, t41863, t41865)
}
