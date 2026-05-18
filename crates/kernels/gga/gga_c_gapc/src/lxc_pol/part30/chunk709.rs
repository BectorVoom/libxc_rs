//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 709/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk709<F: Float>(t2941: F, t8376: F, t101: F, t1762: F, t4533: F, t1947: F, t2937: F, t1928: F, t1936: F, t1587: F, t2880: F, t2879: F) -> (F, F, F, F, F, F) {
    let t8377 = t2941 * t8376;
    let t8379 = t1762 * t101;
    let t8380 = t8379 * t4533;
    let t8381 = t2937 * t1947;
    let t8382 = t8380 * t8381;
    let t8384 = t1936 * t1928;
    let t8385 = t2941 * t8384;
    let t8387 = t2880 * t1587;
    let t8388 = t2879 * t8387;
    (t8377, t8379, t8381, t8382, t8385, t8388)
}
