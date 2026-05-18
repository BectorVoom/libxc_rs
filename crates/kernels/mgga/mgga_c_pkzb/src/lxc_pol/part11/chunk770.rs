//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 770/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk770<F: Float>(t2602: F, t5257: F, t2655: F, t175: F, t5255: F, t2590: F) -> (F, F, F, F) {
    let t6873 = t5257 * t2602;
    let t6885 = t5257 * t2655;
    let t6891 = t5255 * t175;
    let t6892 = t2590 * t6891;
    (t6873, t6885, t6891, t6892)
}
