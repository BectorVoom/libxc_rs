//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1191/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1191<F: Float>(t35283: F, t35285: F, t35293: F, t35298: F, t35302: F, t35307: F, t35309: F, t35312: F, t35316: F, t35319: F, t35323: F, t35325: F, t35328: F, t35330: F, t35334: F, t35336: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37265 = 0.16038463156432184077e-5 * t35283;
    let t37266 = 0.12661944597183303218e-6 * t35285;
    let t37269 = 0.18937162934584967535e-3 * t35293;
    let t37270 = 0.18937162934584967535e-3 * t35298;
    let t37273 = 0.21642471925239962898e-3 * t35302;
    let t37275 = 0.16867947048611111112e-5 * t35307;
    let t37276 = 0.80966145833333333338e-4 * t35309;
    let t37277 = 0.48917046440972222224e-4 * t35312;
    let t37278 = 0.38647271295071362317e-7 * t35316;
    let t37279 = 0.14843793402777777779e-3 * t35319;
    let t37280 = 0.4919817889178240741e-6 * t35323;
    let t37281 = 0.61551119569641057312e-8 * t35325;
    let t37282 = 0.17952409874478641716e-8 * t35328;
    let t37283 = 0.21720231316129303386e-4 * t35330;
    let t37285 = 0.11594181388521408695e-4 * t35334;
    let t37286 = 0.2318836277704281739e-4 * t35336;
    (t37265, t37266, t37269, t37270, t37273, t37275, t37276, t37277, t37278, t37279, t37280, t37281, t37282, t37283, t37285, t37286)
}
