//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1202/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1202<F: Float>(t35254: F, t35257: F, t35259: F, t35263: F, t35269: F, t35272: F, t35275: F, t35277: F, t35280: F, t35283: F, t35285: F, t35293: F, t35298: F, t35287: F, t35289: F, t35302: F) -> (F, F, F, F, F, F) {
    let t37254 = 0.36652500116630512966e-6 * t35254;
    let t37255 = 0.41030519691600762993e-3 * t35257;
    let t37256 = 0.94685814672924837674e-4 * t35259;
    let t37257 = 0.1500544456199363426e-4 * t35263;
    let t37260 = 0.84412963981222021456e-7 * t35269;
    let t37261 = 0.80045999977926802214e-7 * t35272;
    let t37262 = 0.80192315782160920384e-6 * t35275;
    let t37263 = 0.20517039856547019104e-8 * t35277;
    let t37264 = 0.33816362383187442026e-5 * t35280;
    let t37265 = 0.16038463156432184077e-5 * t35283;
    let t37266 = 0.12661944597183303218e-6 * t35285;
    let t37269 = 0.18937162934584967535e-3 * t35293;
    let t37270 = 0.18937162934584967535e-3 * t35298;
    let t37271 = t37260 + t37261 + t37262 - t37263 - t37264 + t37265 + t37266 - 0.38673709012042260327e-7 * t35287 - 0.54083013361612955739e-6 * t35289 - t37269 + t37270;
    let t37273 = 0.21642471925239962898e-3 * t35302;
    (t37254, t37255, t37256, t37257, t37271, t37273)
}
