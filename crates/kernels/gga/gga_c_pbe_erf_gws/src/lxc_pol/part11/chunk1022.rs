//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1022/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1022<F: Float>(t12468: F, t587: F, t7669: F, t12800: F, t1820: F, t12729: F, t572: F, t12484: F, t172: F, t184: F, t10300: F, t2612: F) -> (F, F, F, F, F) {
    let t41769 = t587 * t7669 * t12468;
    let t41772 = t1820 * t7669 * t12800;
    let t41787 = t12729 * t572;
    let t41840 = t172 * t12484 * t184;
    let t41847 = t2612 * t10300;
    (t41769, t41772, t41787, t41840, t41847)
}
