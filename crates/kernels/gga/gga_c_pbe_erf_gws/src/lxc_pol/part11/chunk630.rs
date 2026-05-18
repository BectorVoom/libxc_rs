//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 630/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk630<F: Float>(t168: F, t270: F, t5589: F, t153: F, t274: F, t4573: F, t147: F, t285: F, t4576: F, t281: F, t131: F, t2029: F) -> (F, F, F, F, F) {
    let t5592 = F::new(0.19455129084526283664e0) * t168 * t5589 * t270;
    let t5595 = F::new(0.4429070076315393047e1) * t153 * t4573 * t274;
    let t5615 = t147 * t4576 * t285;
    let t5617 = F::new(0.11974234010254609094e-1) * t281 * t5615;
    let t5621 = F::new(1.0) / t2029 / t131;
    (t5592, t5595, t5615, t5617, t5621)
}
