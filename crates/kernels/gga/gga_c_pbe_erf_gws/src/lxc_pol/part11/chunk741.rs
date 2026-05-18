//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 741/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk741<F: Float>(t12068: F, t856: F, t1076: F, t1112: F, t3820: F, t6484: F, t3067: F, t3721: F, t829: F, t830: F) -> (F, F, F, F) {
    let t12069 = t856 * t12068;
    let t12072 = t1112 * t1076;
    let t12092 = t6484 * t3820;
    let t12109 = t3067 * t3721;
    let t12111 = t829 * t830 * t12109;
    (t12069, t12072, t12092, t12111)
}
