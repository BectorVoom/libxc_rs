//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 649/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk649<F: Float>(t2138: F, t3123: F, t1125: F, t2142: F, t1114: F, t2145: F, t2150: F, t1133: F, t5: F) -> (F, F, F, F, F) {
    let t3125 = t3123 * t2138 / F::cast_from(96.0_f64);
    let t3126 = t1125 * t2142;
    let t3127 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t3126;
    let t3128 = t1114 * t2145;
    let t3130 = t3128 * t2150 / F::cast_from(48.0_f64);
    let t3131 = t5 * t1133;
    (t3125, t3127, t3128, t3130, t3131)
}
