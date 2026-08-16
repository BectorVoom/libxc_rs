//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 779/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk779<F: Float>(t1368: F, t285: F, t535: F, t281: F, t147: F, t4576: F, t131: F, t2029: F, t168: F, t5589: F, t286: F, t137: F, t142: F) -> (F, F, F, F, F) {
    let t5611 = t535 * t1368 * t285;
    let t5612 = t281 * t5611;
    let t5615 = t147 * t4576 * t285;
    let t5617 = F::cast_from(0.11974234010254609094e-1_f64) * t281 * t5615;
    let t5621 = F::cast_from(1.0_f64) / t2029 / t131;
    let t5631 = t168 * t5589;
    let t5633 = F::cast_from(0.19513566535229733338e0_f64) * t5631 * t286;
    let t5651 = t137 * t142;
    (t5612, t5617, t5621, t5633, t5651)
}
