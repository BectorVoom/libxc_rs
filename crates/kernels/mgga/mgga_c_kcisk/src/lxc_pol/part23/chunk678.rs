//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 678/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk678<F: Float>(t1341: F, t5991: F, t3785: F, t1411: F, t1286: F, t2231: F) -> (F, F, F, F) {
    let t5992 = t1341 * t5991;
    let t5993 = t3785 * t5992;
    let t5994 = t1411 * t5993;
    let t5996 = t2231 * t1286;
    (t5992, t5993, t5994, t5996)
}
