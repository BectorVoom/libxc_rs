//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 646/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk646<F: Float>(t169: F, t2994: F, t700: F, t784: F, t991: F, t242: F, t1086: F, t1383: F, t1371: F, t2948: F, t553: F, t1378: F, t1971: F, t1049: F, t1986: F, t2007: F, t2970: F) -> (F, F, F, F, F, F, F, F) {
    let t8357 = t169 * t2994 * t700;
    let t8361 = t784 * t991;
    let t8363 = t169 * t8361 * t242;
    let t8373 = t169 * t1086 * t1383;
    let t8387 = t2948 * t1371 * t553;
    let t8390 = t8361 * t1378 * t1971;
    let t8405 = t1049 * t1986;
    let t8408 = t2970 * t2007;
    (t8357, t8361, t8363, t8373, t8387, t8390, t8405, t8408)
}
