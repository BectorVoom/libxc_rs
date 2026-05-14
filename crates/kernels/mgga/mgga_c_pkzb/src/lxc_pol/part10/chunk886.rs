//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 886/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk886<F: Float>(t2226: F, t394: F, t1478: F, t154: F, t386: F, t385: F, t465: F, t931: F) -> (F, F, F, F) {
    let t6367 = t394 * t2226;
    let t6377 = t154 * t1478 * t386;
    let t6379 = 5.0 / 1296.0 * t385 * t6377;
    let t6380 = t465 * t931;
    (t6367, t6377, t6379, t6380)
}
