//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 542/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk542<F: Float>(t138: F, t1577: F, t2902: F, t3671: F, t3675: F, t3683: F, t514: F, t985: F, t101: F) -> (F, F) {
    let t3685 = t138 * t3671 + F::cast_from(2.0_f64) * t1577 * t3675 - F::cast_from(2.0_f64) * t2902 * t985 - t3683 * t514;
    let t3686 = t101 * t3685;
    (t3685, t3686)
}
