//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 604/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk604<F: Float>(t1411: F, t5993: F, t1286: F, t2231: F, t1450: F, t1415: F, t1440: F, t2152: F) -> (F, F, F, F, F, F) {
    let t5994 = t1411 * t5993;
    let t5996 = t2231 * t1286;
    let t5997 = t1450 * t5996;
    let t5998 = t1415 * t5997;
    let t5999 = t1411 * t5998;
    let t6001 = t2152 * t1440;
    (t5994, t5996, t5997, t5998, t5999, t6001)
}
