//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 710/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk710<F: Float>(t202: F, t3477: F, t184: F, t3345: F, t572: F, t1663: F, t3346: F, t1022: F, t7483: F, t3530: F, t5283: F, t587: F) -> (F, F, F, F, F, F, F) {
    let t10418 = t202 * t3477;
    let t10419 = t10418 * t184;
    let t10424 = t3345 * t572;
    let t10442 = t1663 * t3346;
    let t10465 = t7483 * t1022;
    let t10472 = t5283 * t3530;
    let t10473 = t587 * t10472;
    (t10418, t10419, t10424, t10442, t10465, t10472, t10473)
}
