//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1418/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1418<F: Float>(t35280: F, t35283: F, t35285: F, t35293: F, t35298: F, t35287: F, t35289: F, t37260: F, t37261: F, t37262: F, t37263: F, t35302: F) -> (F, F) {
    let t37264 = F::cast_from(0.33816362383187442026e-5_f64) * t35280;
    let t37265 = F::cast_from(0.16038463156432184077e-5_f64) * t35283;
    let t37266 = F::cast_from(0.12661944597183303218e-6_f64) * t35285;
    let t37269 = F::cast_from(0.18937162934584967535e-3_f64) * t35293;
    let t37270 = F::cast_from(0.18937162934584967535e-3_f64) * t35298;
    let t37271 = t37260 + t37261 + t37262 - t37263 - t37264 + t37265 + t37266 - F::cast_from(0.38673709012042260327e-7_f64) * t35287 - F::cast_from(0.54083013361612955739e-6_f64) * t35289 - t37269 + t37270;
    let t37273 = F::cast_from(0.21642471925239962898e-3_f64) * t35302;
    (t37271, t37273)
}
