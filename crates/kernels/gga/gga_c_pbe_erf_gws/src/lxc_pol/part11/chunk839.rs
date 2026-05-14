//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 839/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk839<F: Float>(t168: F, t18344: F, t286: F, t147: F, t18049: F, t281: F, t285: F, t545: F, t5984: F, t159: F, t4259: F, t5631: F, t759: F, t4576: F, t762: F, t413: F, t5772: F, t5773: F) -> (F, F, F, F, F, F, F) {
    let t19121 = 0.91063310497738755577e0 * t168 * t18344 * t286;
    let t19157 = 0.11974234010254609094e-1 * t281 * t147 * t18049 * t285;
    let t19169 = 0.26861343269868796571e-1 * t5984 * t545 * t285;
    let t19174 = 0.10943113336969376162e-5 * t4259 * t147 * t159 * t285;
    let t19182 = 0.78054266140918933351e0 * t5631 * t759;
    let t19203 = 0.11622696607154767747e-2 * t762 * t4576 * t285;
    let t19229 = 0.15589466666666666666e2 * t5772 * t5773 * t413;
    (t19121, t19157, t19169, t19174, t19182, t19203, t19229)
}
