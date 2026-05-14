//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1119/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1119<F: Float>(t32000: F, t9462: F, t1339: F, t3278: F, t5625: F, t9461: F, t21499: F, t9425: F) -> (F, F, F, F, F, F) {
    let t32001 = t32000 * t9462;
    let t32002 = t1339 * t32001;
    let t32004 = t5625 * t3278;
    let t32005 = t9461 * t32004;
    let t32006 = t1339 * t32005;
    let t32008 = t9425 * t21499;
    (t32001, t32002, t32004, t32005, t32006, t32008)
}
