//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1203/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1203<F: Float>(t35307: F, t35309: F, t35312: F, t35316: F, t35319: F, t35323: F, t35325: F, t35328: F, t35330: F, t35304: F, t37273: F, t35334: F, t35336: F, t35339: F, t35349: F, t35352: F) -> (F, F, F, F, F, F) {
    let t37275 = 0.16867947048611111112e-5 * t35307;
    let t37276 = 0.80966145833333333338e-4 * t35309;
    let t37277 = 0.48917046440972222224e-4 * t35312;
    let t37278 = 0.38647271295071362317e-7 * t35316;
    let t37279 = 0.14843793402777777779e-3 * t35319;
    let t37280 = 0.4919817889178240741e-6 * t35323;
    let t37281 = 0.61551119569641057312e-8 * t35325;
    let t37282 = 0.17952409874478641716e-8 * t35328;
    let t37283 = 0.21720231316129303386e-4 * t35330;
    let t37284 = t37273 - 0.68832926096598307304e-7 * t35304 - t37275 - t37276 + t37277 - t37278 + t37279 + t37280 + t37281 + t37282 + t37283;
    let t37285 = 0.11594181388521408695e-4 * t35334;
    let t37286 = 0.2318836277704281739e-4 * t35336;
    let t37287 = 0.11594181388521408695e-4 * t35339;
    let t37291 = 0.19120257249055085362e-8 * t35349;
    let t37292 = 0.12310223913928211462e-7 * t35352;
    (t37284, t37285, t37286, t37287, t37291, t37292)
}
