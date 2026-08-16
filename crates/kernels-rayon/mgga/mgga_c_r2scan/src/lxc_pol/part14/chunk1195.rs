//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1195/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1195(t12210: f64, t37346: f64, t11506: f64, t38697: f64, t10626: f64, t12056: f64, t3275: f64, t11458: f64, t40282: f64, t38715: f64, t40394: f64, t11455: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41256 = 3.0_f64 / 4.0_f64 * t37346 * t12210;
    let t41258 = 3.0_f64 / 4.0_f64 * t11506 * t38697;
    let t41261 = t3275 * t12056 * t10626 / 2.0_f64;
    let t41263 = 3.0_f64 / 2.0_f64 * t40282 * t11458;
    let t41265 = 3.0_f64 / 2.0_f64 * t40394 * t38715;
    let t41270 = 15.0_f64 / 8.0_f64 * t40282 * t11455;
    (t41256, t41258, t41261, t41263, t41265, t41270)
}
