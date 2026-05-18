//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1131/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1131<F: Float>(t32523: F, t12538: F, t2615: F, t16932: F, t47391: F, t5293: F, t587: F, t12821: F, t23123: F, t5211: F, t41326: F, t12783: F, t2612: F) -> (F, F, F, F, F, F) {
    let t48050 = F::new(32.0) / F::new(45.0) * t32523;
    let t48052 = F::new(32.0) / F::new(9.0) * t2615 * t12538;
    let t48056 = F::new(128.0) / F::new(27.0) * t587 * t5293 * t16932 * t47391;
    let t48059 = F::new(64.0) / F::new(15.0) * t5211 * t23123 * t12821;
    let t48060 = F::new(32.0) / F::new(45.0) * t41326;
    let t48062 = F::new(16.0) / F::new(15.0) * t2612 * t12783;
    (t48050, t48052, t48056, t48059, t48060, t48062)
}
