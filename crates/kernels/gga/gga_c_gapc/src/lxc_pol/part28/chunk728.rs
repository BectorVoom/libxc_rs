//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 728/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk728<F: Float>(t4533: F, t8379: F, t1947: F, t2937: F, t1928: F, t1936: F, t2941: F, t1587: F, t2880: F, t2879: F, t2885: F, t507: F) -> (F, F, F, F, F) {
    let t8380 = t8379 * t4533;
    let t8381 = t2937 * t1947;
    let t8382 = t8380 * t8381;
    let t8384 = t1936 * t1928;
    let t8385 = t2941 * t8384;
    let t8387 = t2880 * t1587;
    let t8388 = t2879 * t8387;
    let t8390 = t2885 * t507;
    (t8381, t8382, t8385, t8388, t8390)
}
