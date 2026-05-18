//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1409/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1409<F: Float>(t35272: F, t35275: F, t35277: F, t35280: F, t35283: F, t35285: F, t35293: F, t35298: F, t35302: F, t35307: F, t35309: F, t35312: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37261 = F::new(0.80045999977926802214e-7) * t35272;
    let t37262 = F::new(0.80192315782160920384e-6) * t35275;
    let t37263 = F::new(0.20517039856547019104e-8) * t35277;
    let t37264 = F::new(0.33816362383187442026e-5) * t35280;
    let t37265 = F::new(0.16038463156432184077e-5) * t35283;
    let t37266 = F::new(0.12661944597183303218e-6) * t35285;
    let t37269 = F::new(0.18937162934584967535e-3) * t35293;
    let t37270 = F::new(0.18937162934584967535e-3) * t35298;
    let t37273 = F::new(0.21642471925239962898e-3) * t35302;
    let t37275 = F::new(0.16867947048611111112e-5) * t35307;
    let t37276 = F::new(0.80966145833333333338e-4) * t35309;
    let t37277 = F::new(0.48917046440972222224e-4) * t35312;
    (t37261, t37262, t37263, t37264, t37265, t37266, t37269, t37270, t37273, t37275, t37276, t37277)
}
