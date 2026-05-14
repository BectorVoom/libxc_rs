//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 671/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk671<F: Float>(t8489: F, t8535: F, t1404: F, t493: F, t2928: F, t2912: F, t8459: F, t2929: F, t2941: F, t1845: F, t515: F, t996: F, t4538: F, t1599: F, t2932: F, t2958: F) -> (F, F, F, F, F, F, F) {
    let t8536 = t8489 * t8535;
    let t8538 = t493 * t1404;
    let t8539 = t2928 * t8538;
    let t8541 = t8459 * t2912;
    let t8543 = t2941 * t2929;
    let t8545 = t1845 * t515;
    let t8546 = t996 * t8545;
    let t8547 = t8546 * t2912;
    let t8549 = t996 * t4538;
    let t8550 = t8549 * t2929;
    let t8552 = t2932 * t1599;
    let t8553 = t8552 * t2958;
    (t8536, t8539, t8541, t8543, t8547, t8550, t8553)
}
