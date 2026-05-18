//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 855/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk855<F: Float>(t5309: F, t5312: F, t1820: F, t1885: F, t5333: F, t597: F, t610: F, t5019: F, t5515: F, t586: F, t593: F, t1656: F, t5304: F) -> (F, F, F, F, F) {
    let t16515 = F::new(16.0) / F::new(5.0) * t5312 * t5309;
    let t16520 = F::new(16.0) / F::new(15.0) * t1820 * t1885 * t597 * t5333 * t610;
    let t16521 = t5312 * t5019;
    let t16522 = F::new(64.0) / F::new(15.0) * t16521;
    let t16523 = t5515 * t586;
    let t16525 = F::new(32.0) / F::new(15.0) * t16523 * t593;
    let t16527 = F::new(16.0) / F::new(15.0) * t5304 * t1656;
    (t16515, t16520, t16522, t16525, t16527)
}
