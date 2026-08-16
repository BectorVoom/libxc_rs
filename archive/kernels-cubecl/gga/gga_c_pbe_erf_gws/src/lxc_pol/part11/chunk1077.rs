//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1077/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1077<F: Float>(t12723: F, t30876: F, t1820: F, t1821: F, t41787: F, t950: F, t10442: F, t3342: F, t587: F, t12464: F, t2559: F, t995: F) -> (F, F, F, F, F) {
    let t47343 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t30876 * t12723;
    let t47347 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t1820 * t1821 * t41787 * t950;
    let t47348 = t10442 * t3342;
    let t47351 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t587 * t1821 * t47348;
    let t47355 = F::cast_from(64.0_f64) / F::cast_from(9.0_f64) * t1820 * t2559 * t12464 * t995;
    (t47343, t47347, t47348, t47351, t47355)
}
