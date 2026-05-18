//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1102/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1102<F: Float>(t14079: F, t918: F, t1477: F, t326: F, t346: F, t1185: F, t2339: F, t4039: F, t2273: F, t2278: F, t850: F, t852: F) -> (F, F, F, F, F, F, F) {
    let t14080 = t14079 * t918;
    let t14081 = F::new(7.0) / F::new(576.0) * t14080;
    let t14083 = t326 * t346 * t1477;
    let t14084 = t14083 * t1185;
    let t14086 = t4039 * t2339;
    let t14088 = t4039 * t2273;
    let t14091 = t850 * t2278 * t852;
    (t14080, t14081, t14083, t14084, t14086, t14088, t14091)
}
