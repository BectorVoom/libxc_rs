//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1040/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1040(t36511: f64, t36513: f64, t1664: f64, t2127: f64, t16156: f64, t9055: f64, t2085: f64, t8339: f64, t1162: f64, t1979: f64, t1982: f64, t201: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41647 = 0.19863479950205658386e-3_f64 * t36511;
    let t41648 = 0.19863479950205658386e-3_f64 * t36513;
    let t41651 = t1664 * t2127;
    let t41654 = t16156 * t9055;
    let t41656 = t8339 * t2085;
    let t41657 = 0.18183107769496894486e-1_f64 * t41656;
    let t41663 = t589 * t1162 * t201 * t1979 * t1982;
    (t41647, t41648, t41651, t41654, t41657, t41663)
}
