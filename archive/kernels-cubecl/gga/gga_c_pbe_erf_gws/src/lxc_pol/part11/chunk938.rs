//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 938/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk938<F: Float>(t2276: F, t2299: F, t6201: F, t20269: F, t932: F, t2200: F, t863: F, t864: F, t322: F, t6382: F, t274: F, t6094: F) -> (F, F, F, F, F) {
    let t20944 = t2276 * t6201 * t2299;
    let t20948 = t2276 * t20269 * t932;
    let t20962 = t863 * t864 * t2200;
    let t21010 = t322 * t6382;
    let t21091 = t322 / t6094 / t274;
    (t20944, t20948, t20962, t21010, t21091)
}
