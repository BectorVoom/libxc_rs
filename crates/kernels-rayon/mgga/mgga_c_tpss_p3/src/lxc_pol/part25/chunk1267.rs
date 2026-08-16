//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1267/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1267(t3205: f64, t5935: f64, t36: f64, t68: f64, t1338: f64, t1795: f64, t1799: f64, t6435: f64, t1289: f64, t1270: f64, t3204: f64, t10178: f64, t536: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24128 = t3205 * t5935;
    let t24289 = t68 * t36;
    let t25232 = t1795 * t1338;
    let t25315 = t1338 * t1799;
    let t25469 = t3205 * t6435;
    let t25752 = t24289 * t1289;
    let t26848 = t6435 * t1270;
    let t30366 = t3204 * t3204;
    let t30367 = 1.0_f64 / t30366;
    let t31297 = 1.0_f64 / t10178 / t536;
    (t24128, t25232, t25315, t25469, t25752, t26848, t30367, t31297)
}
