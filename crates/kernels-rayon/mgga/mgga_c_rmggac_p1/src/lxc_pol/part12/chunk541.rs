//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 541/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk541(t132: f64, t1341: f64, t638: f64, t7310: f64, t1249: f64, t511: f64, t650: f64, t2085: f64, t2181: f64, t33: f64, t78: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7311 = t132 * t1341;
    let t7313 = t638 * t7310 * t7311;
    let t7315 = t1249 * t511;
    let t7316 = t7315 * t650;
    let t7317 = 0.34093327067806677161e-2_f64 * t7316;
    let t7318 = t2181 * t2085;
    let t7320 = t78 * t33;
    let t7321 = 1.0_f64 / t7320;
    let t7322 = t7321 * t271;
    (t7311, t7313, t7315, t7317, t7318, t7321, t7322)
}
