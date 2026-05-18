//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 419/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk419<F: Float>(t1368: F, t147: F, t285: F, t281: F, t520: F, t524: F, t142: F, t100: F, t95: F, t481: F) -> (F, F, F, F, F, F) {
    let t1497 = t147 * t1368 * t285;
    let t1499 = F::new(0.11974234010254609094e-1) * t281 * t1497;
    let t1500 = t524 * t520;
    let t1501 = t1500 * t142;
    let t1503 = t95 * t100;
    let t1504 = t481 * t481;
    (t1497, t1499, t1500, t1501, t1503, t1504)
}
