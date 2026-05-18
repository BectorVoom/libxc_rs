//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 952/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk952<F: Float>(t1903: F, t2650: F, t2654: F, t5441: F, t723: F, t7733: F, t5434: F, t2735: F, t561: F, t996: F, t1022: F, t7116: F) -> (F, F, F, F, F, F) {
    let t22986 = t2650 * t1903;
    let t22988 = t2654 * t5441;
    let t22994 = t7733 * t723;
    let t22996 = t2654 * t5434;
    let t23109 = t561 * t2735 * t996;
    let t23123 = t7116 * t1022;
    (t22986, t22988, t22994, t22996, t23109, t23123)
}
