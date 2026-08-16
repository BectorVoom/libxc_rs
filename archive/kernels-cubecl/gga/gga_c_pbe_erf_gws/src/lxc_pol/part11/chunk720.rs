//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 720/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk720<F: Float>(t10927: F, t639: F, t3512: F, t5493: F, t1620: F, t2612: F, t2640: F, t3443: F, t572: F, t172: F, t3486: F, t184: F) -> (F, F, F, F, F, F, F) {
    let t10928 = t639 * t10927;
    let t10930 = t5493 * t3512;
    let t10931 = t1620 * t10930;
    let t10933 = t2612 * t2640;
    let t10938 = t3443 * t572;
    let t10968 = t172 * t3486;
    let t10969 = t10968 * t184;
    (t10928, t10930, t10931, t10933, t10938, t10968, t10969)
}
