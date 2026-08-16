//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1169/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1169(t1259: f64, t13108: f64, t10171: f64, t1256: f64, t1266: f64, t13033: f64, t13035: f64, t13047: f64, t13051: f64, t13055: f64, t1657: f64, t3360: f64, t3367: f64, t3385: f64, t4490: f64, t4494: f64, t4517: f64, t538: f64) -> (f64, f64) {
    let t13109 = t1259 * t13108;
    let t13111 = -t10171 * t1657 - 6.0_f64 * t1256 * t13047 + 4.0_f64 * t1256 * t13051 + 2.0_f64 * t1256 * t13055 - t1256 * t13109 - 2.0_f64 * t1266 * t13035 + t13033 * t538 + 4.0_f64 * t3360 * t4494 - 2.0_f64 * t3360 * t4517 + 2.0_f64 * t3367 * t4490 - t3385 * t4490;
    (t13109, t13111)
}
