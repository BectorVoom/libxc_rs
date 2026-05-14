//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 946/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk946<F: Float>(t1528: F, t16986: F, t4370: F, t4373: F, t16978: F, t478: F, t19059: F, t19062: F, t19064: F, t19066: F, t19068: F, t19072: F, t19075: F, t522: F, t5621: F, t475: F) -> (F, F, F, F, F, F) {
    let t19077 = t1528 * t16986;
    let t19079 = t4370 * t4373;
    let t19081 = t478 * t16978;
    let t19083 = -28.0 / 81.0 * t19059 + 8.0 / 9.0 * t19062 - t19064 / 3.0 - 4.0 / 9.0 * t19066 + t19068 / 3.0 - 28.0 / 81.0 * t19072 + 8.0 / 9.0 * t19075 - t19077 / 3.0 - 4.0 / 9.0 * t19079 + t19081 / 3.0;
    let t19087 = t522 * t5621;
    let t19088 = t475 * t19087;
    (t19077, t19079, t19081, t19083, t19087, t19088)
}
