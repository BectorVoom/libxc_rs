//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 665/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk665<F: Float>(t1592: F, t475: F, t1503: F, t522: F, t142: F, t1504: F, t525: F, t1354: F, t285: F, t545: F, t281: F, t1368: F, t535: F, t147: F, t4576: F, t131: F, t2029: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5598 = t475 * t1592;
    let t5601 = t1503 * t522;
    let t5602 = t142 * t1504;
    let t5603 = t525 * t5602;
    let t5607 = t1354 * t545 * t285;
    let t5608 = t281 * t5607;
    let t5611 = t535 * t1368 * t285;
    let t5612 = t281 * t5611;
    let t5615 = t147 * t4576 * t285;
    let t5617 = 0.11974234010254609094e-1 * t281 * t5615;
    let t5621 = 1.0 / t2029 / t131;
    (t5598, t5601, t5602, t5603, t5607, t5608, t5611, t5612, t5615, t5617, t5621)
}
