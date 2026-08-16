//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1419/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1419(t35307: f64, t35309: f64, t35312: f64, t35316: f64, t35319: f64, t35323: f64, t35325: f64, t35328: f64, t35330: f64, t35304: f64, t37273: f64, t35334: f64) -> (f64, f64) {
    let t37275 = 0.16867947048611111112e-5_f64 * t35307;
    let t37276 = 0.80966145833333333338e-4_f64 * t35309;
    let t37277 = 0.48917046440972222224e-4_f64 * t35312;
    let t37278 = 0.38647271295071362317e-7_f64 * t35316;
    let t37279 = 0.14843793402777777779e-3_f64 * t35319;
    let t37280 = 0.4919817889178240741e-6_f64 * t35323;
    let t37281 = 0.61551119569641057312e-8_f64 * t35325;
    let t37282 = 0.17952409874478641716e-8_f64 * t35328;
    let t37283 = 0.21720231316129303386e-4_f64 * t35330;
    let t37284 = t37273 - 0.68832926096598307304e-7_f64 * t35304 - t37275 - t37276 + t37277 - t37278 + t37279 + t37280 + t37281 + t37282 + t37283;
    let t37285 = 0.11594181388521408695e-4_f64 * t35334;
    (t37284, t37285)
}
