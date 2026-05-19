//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 695/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk695<F: Float>(t281: F, t5611: F, t147: F, t285: F, t4576: F, t131: F, t2029: F, t137: F, t510: F, t142: F, t1570: F, t2031: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5612 = t281 * t5611;
    let t5615 = t147 * t4576 * t285;
    let t5617 = F::cast_from(0.11974234010254609094e-1_f64) * t281 * t5615;
    let t5621 = F::new(1.0) / t2029 / t131;
    let t5622 = t5621 * t137;
    let t5623 = t510 * t510;
    let t5624 = t142 * t5623;
    let t5625 = t5622 * t5624;
    let t5628 = t142 * t1570;
    let t5629 = t2031 * t5628;
    (t5612, t5615, t5617, t5621, t5623, t5624, t5625, t5628, t5629)
}
