//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 726/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk726<F: Float>(t190: F, t200: F, t1954: F, t8489: F, t1404: F, t493: F, t2928: F, t2912: F, t8459: F, t2929: F, t2941: F, t1845: F, t515: F) -> (F, F, F, F, F) {
    let t8534 = t190 * t200;
    let t8535 = t8534 * t1954;
    let t8536 = t8489 * t8535;
    let t8538 = t493 * t1404;
    let t8539 = t2928 * t8538;
    let t8541 = t8459 * t2912;
    let t8543 = t2941 * t2929;
    let t8545 = t1845 * t515;
    (t8536, t8539, t8541, t8543, t8545)
}
