//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 677/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk677<F: Float>(t2899: F, t385: F, t1510: F, t2880: F, t120: F, t3954: F, t436: F, t2941: F, t101: F, t1762: F, t4533: F, t1947: F, t2937: F, t1928: F, t1936: F, t1587: F) -> (F, F, F, F, F, F, F, F) {
    let t8371 = t385 * t2899;
    let t8373 = t2880 * t1510;
    let t8374 = t120 * t8373;
    let t8376 = t436 * t3954;
    let t8377 = t2941 * t8376;
    let t8379 = t1762 * t101;
    let t8380 = t8379 * t4533;
    let t8381 = t2937 * t1947;
    let t8382 = t8380 * t8381;
    let t8384 = t1936 * t1928;
    let t8385 = t2941 * t8384;
    let t8387 = t2880 * t1587;
    (t8371, t8374, t8377, t8379, t8381, t8382, t8385, t8387)
}
