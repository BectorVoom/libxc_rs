//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1023/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1023<F: Float>(t3803: F, t6631: F, t3257: F, t3028: F, t5: F, t337: F, t2121: F, t3116: F, t3854: F, t2170: F, t2171: F, t2168: F) -> (F, F, F, F, F, F, F) {
    let t11450 = t3803 * t6631;
    let t11451 = t3257 * t11450;
    let t11454 = t5 * t3028;
    let t11455 = t337 * t11454;
    let t11456 = t2121 * t11455;
    let t11458 = t3116 * t11456 / F::new(96.0);
    let t11459 = t5 * t3854;
    let t11461 = t2170 * t11459 * t2171;
    let t11463 = t2168 * t11461 / F::new(48.0);
    (t11450, t11451, t11455, t11458, t11459, t11461, t11463)
}
