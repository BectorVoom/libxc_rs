//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 868/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk868<F: Float>(t1971: F, t4585: F, t8361: F, t553: F, t8309: F, t1371: F, t8465: F, t3013: F, t547: F, t164: F, t8279: F, t26143: F, t1052: F, t163: F, t169: F, t366: F) -> (F, F, F, F, F, F, F) {
    let t26402 = t8361 * t4585 * t1971;
    let t26404 = t8309 * t553;
    let t26411 = t8465 * t1371 * t553;
    let t26415 = t3013 * t547;
    let t26417 = t8279 * t164;
    let t26419 = t26143 * t164;
    let t26432 = t169 * t366 * t1052 * t163;
    (t26402, t26404, t26411, t26415, t26417, t26419, t26432)
}
