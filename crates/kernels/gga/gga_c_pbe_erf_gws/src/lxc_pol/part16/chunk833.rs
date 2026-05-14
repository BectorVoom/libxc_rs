//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 833/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk833<F: Float>(t1076: F, t1365: F, t153: F, t2513: F, t414: F, t4547: F, t1333: F, t960: F, t1438: F, t2515: F, t409: F, t4602: F, t4503: F, t4506: F, t4513: F, t4539: F, t4542: F, t4744: F, t6918: F, t6932: F, t6966: F, t6969: F) -> (F, F, F, F, F, F, F, F) {
    let t7981 = t153 * t1365 * t1076;
    let t7983 = t414 * t2513;
    let t7984 = 8.0 * t7983;
    let t7985 = 4.0 * t4547;
    let t7986 = t1333 * t960;
    let t7987 = 20.0 * t7986;
    let t7988 = t1438 * t960;
    let t7989 = 32.0 * t7988;
    let t7990 = t409 * t2515;
    let t7991 = 8.0 * t7990;
    let t7992 = 2.0 * t4602;
    let t7993 = t6918 + t4503 - t4506 - t4513 + t4539 + t4542 + t6932 + t6966 + t6969 - t7984 - t7985 + t7987 - t7989 + t7991 + t7992 + t4744;
    (t7981, t7984, t7985, t7987, t7989, t7991, t7992, t7993)
}
