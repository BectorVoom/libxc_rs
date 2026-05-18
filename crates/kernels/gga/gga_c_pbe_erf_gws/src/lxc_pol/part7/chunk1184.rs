//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1184/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1184<F: Float>(t2298: F, t814: F, t322: F, t6382: F, t2182: F, t19505: F, t19553: F, t2074: F, t20992: F, t20995: F, t20998: F, t2178: F, t2181: F, t2183: F, t2186: F, t339: F, t340: F, t4379: F, t6421: F, t6424: F, t6429: F, t6430: F, t6433: F, t6436: F, t870: F, t871: F) -> (F, F) {
    let t21003 = t814 * t2298;
    let t21010 = t322 * t6382;
    let t21011 = t2182 * t2182;
    let t21027 = -F::new(36.0) * t19505 * t2181 * t339 + F::new(3.0) * t19553 * t339 * t870 + F::new(360.0) * t2074 * t2183 * t6429 - t20992 * t339 * t340 - F::new(360.0) * t21010 * t21011 * t339 - F::new(48.0) * t2181 * t4379 * t871 + F::new(12.0) * t20995 * t871 - F::new(72.0) * t20998 * t2183 + F::new(240.0) * t21003 * t6430 + F::new(12.0) * t2178 * t6436 + F::new(18.0) * t2186 * t6421 - F::new(144.0) * t6424 * t6433;
    (t21011, t21027)
}
