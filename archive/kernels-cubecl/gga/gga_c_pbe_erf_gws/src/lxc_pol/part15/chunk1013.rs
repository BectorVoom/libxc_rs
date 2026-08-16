//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1013/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1013<F: Float>(t2170: F, t3131: F, t6220: F, t2168: F, t6510: F, t2195: F, t3178: F, t9037: F, t9039: F, t9041: F, t9042: F, t9084: F, t9086: F, t9090: F, t9094: F, t9096: F) -> (F, F, F, F, F, F) {
    let t9098 = t2170 * t3131 * t6220;
    let t9100 = t2168 * t9098 / F::cast_from(48.0_f64);
    let t9101 = F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t6510;
    let t9103 = t2170 * t3178 * t2195;
    let t9105 = t2168 * t9103 / F::cast_from(48.0_f64);
    let t9106 = t9037 - t9039 - t9041 - t9042 - t9084 + t9086 + t9090 + t9094 - t9096 + t9100 - t9101 + t9105;
    (t9098, t9100, t9101, t9103, t9105, t9106)
}
