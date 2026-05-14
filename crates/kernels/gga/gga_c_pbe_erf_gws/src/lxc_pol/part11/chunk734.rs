//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 734/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk734<F: Float>(t3342: F, t7087: F, t1821: F, t1820: F, t1017: F, t3425: F, t1827: F, t587: F, t1044: F, t3465: F, t5522: F, t639: F, t11054: F, t954: F, t4927: F, t7845: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12809 = t7087 * t3342;
    let t12810 = t1821 * t12809;
    let t12812 = 16.0 / 15.0 * t1820 * t12810;
    let t12813 = t3425 * t1017;
    let t12814 = t1827 * t12813;
    let t12816 = 8.0 / 15.0 * t587 * t12814;
    let t12817 = t3465 * t1044;
    let t12818 = t5522 * t12817;
    let t12820 = 4.0 / 9.0 * t639 * t12818;
    let t12821 = t11054 * t954;
    let t12822 = t4927 * t12821;
    let t12824 = 8.0 / 15.0 * t639 * t12822;
    let t12825 = 4.0 / 45.0 * t7845;
    (t12809, t12810, t12812, t12813, t12814, t12816, t12817, t12818, t12820, t12821, t12822, t12824, t12825)
}
