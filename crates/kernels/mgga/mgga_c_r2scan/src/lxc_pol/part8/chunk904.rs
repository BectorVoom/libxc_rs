//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 904/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk904<F: Float>(t2832: F, t784: F, t783: F, t788: F, t2547: F, t6118: F, t571: F, t6148: F, t6621: F, t990: F, t1248: F, t295: F, t1000: F, t6635: F, t1256: F, t305: F) -> (F, F, F, F, F, F, F, F) {
    let t8279 = t2832 * t784;
    let t8282 = 0.11643651550782197811e-1 * t783 * t8279 * t788;
    let t8284 = 0.25610080155860322884e0 * t6118 * t2547;
    let t8289 = t571 * t6148;
    let t8315 = t6621 * t990;
    let t8319 = t295 * t1248;
    let t8336 = t6635 * t1000;
    let t8340 = t305 * t1256;
    (t8279, t8282, t8284, t8289, t8315, t8319, t8336, t8340)
}
