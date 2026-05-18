//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1124/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1124<F: Float>(t14079: F, t918: F, t1477: F, t326: F, t346: F, t1185: F, t4021: F, t828: F) -> (F, F, F, F) {
    let t14080 = t14079 * t918;
    let t14083 = t326 * t346 * t1477;
    let t14084 = t14083 * t1185;
    let t14085 = F::new(35.0) / F::new(432.0) * t14084;
    let t14092 = t4021 * t828;
    (t14080, t14083, t14085, t14092)
}
