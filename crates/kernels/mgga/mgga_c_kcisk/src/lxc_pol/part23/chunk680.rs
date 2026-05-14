//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 680/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk680<F: Float>(t1450: F, t6001: F, t1415: F, t1411: F, t1286: F, t2152: F) -> (F, F, F, F) {
    let t6002 = t1450 * t6001;
    let t6003 = t1415 * t6002;
    let t6004 = t1411 * t6003;
    let t6006 = t2152 * t1286;
    (t6002, t6003, t6004, t6006)
}
