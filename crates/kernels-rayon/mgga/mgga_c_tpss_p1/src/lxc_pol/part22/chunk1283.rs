//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1283/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1283(t3205: f64, t5935: f64, t36: f64, t68: f64, t581: f64, t6435: f64, t1270: f64, t3204: f64, t10178: f64, t536: f64, t1974: f64, t1980: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24128 = t3205 * t5935;
    let t24289 = t68 * t36;
    let t24290 = t24289 * t581;
    let t25469 = t3205 * t6435;
    let t26207 = t5935 * t1270;
    let t30366 = t3204 * t3204;
    let t30367 = 1.0_f64 / t30366;
    let t31297 = 1.0_f64 / t10178 / t536;
    let t31450 = t1974 * t1980;
    (t24128, t24290, t25469, t26207, t30367, t31297, t31450)
}
