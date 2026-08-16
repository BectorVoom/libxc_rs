//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2154/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2154(t87363: f64, t13080: f64, t23146: f64, t242: f64, t812: f64, t81816: f64, t13265: f64, t13333: f64, t25084: f64, t13076: f64, t13084: f64, t87329: f64, t87331: f64, t87333: f64, t87336: f64, t87339: f64, t87342: f64, t87343: f64, t87345: f64, t87348: f64, t87351: f64, t87355: f64, t87359: f64) -> f64 {
    let t87364 = 7.0_f64 / 576.0_f64 * t87363;
    let t87365 = t23146 * t13080;
    let t87368 = t812 * t81816 * t242;
    let t87369 = t87368 * t13265;
    let t87371 = t25084 * t13333;
    let t87373 = t23146 * t13076;
    let t87375 = t25084 * t13084;
    let t87377 = -t87329 + t87331 + t87333 - t87336 + t87339 + t87342 - t87343 / 384.0_f64 - 119.0_f64 / 1728.0_f64 * t87345 - t87348 - 0.84782787797694820792e-2_f64 * t87351 - 0.12111826828242117256e-2_f64 * t87355 - 0.12111826828242117256e-2_f64 * t87359 - t87364 - 5.0_f64 / 384.0_f64 * t87365 - t87369 / 256.0_f64 + t87371 / 256.0_f64 - t87373 / 1536.0_f64 - t87375 / 192.0_f64;
    t87377
}
