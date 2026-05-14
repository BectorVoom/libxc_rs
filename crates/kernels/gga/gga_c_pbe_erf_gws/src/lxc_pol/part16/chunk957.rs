//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 957/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk957<F: Float>(t8628: F, t8667: F, t8721: F, t8772: F, t9211: F, t9273: F, t9320: F, t9737: F, t4383: F, t6158: F, t1114: F, t3222: F, t9607: F, t1143: F, t2416: F, t1105: F, t2053: F) -> (F, F, F, F, F) {
    let t9740 = t8628 + t8667 + t8721 + t8772 + t9211 + t9273 + t9320 + t9737;
    let t11374 = t6158 * t4383;
    let t11375 = t1114 * t11374;
    let t11434 = t9607 * t3222;
    let t12213 = t1143 * t2416;
    let t12275 = t2053 * t1105;
    (t9740, t11375, t11434, t12213, t12275)
}
