//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 677/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk677<F: Float>(t458: F, t998: F, t8524: F, t568: F, t2903: F, t1587: F, t493: F, t2911: F, t190: F, t200: F, t1954: F, t8489: F, t1404: F, t2928: F, t2912: F, t8459: F) -> (F, F, F, F, F, F) {
    let t8525 = t998 * t458;
    let t8526 = t8524 * t8525;
    let t8528 = t998 * t568;
    let t8529 = t2903 * t8528;
    let t8531 = t493 * t1587;
    let t8532 = t2911 * t8531;
    let t8534 = t190 * t200;
    let t8535 = t8534 * t1954;
    let t8536 = t8489 * t8535;
    let t8538 = t493 * t1404;
    let t8539 = t2928 * t8538;
    let t8541 = t8459 * t2912;
    (t8526, t8529, t8532, t8536, t8539, t8541)
}
