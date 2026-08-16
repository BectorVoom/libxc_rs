//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1683/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1683<F: Float>(t4292: F, t6646: F, t1888: F, t2647: F, t4282: F, t22986: F, t6547: F, t7529: F, t25249: F, t829: F, t22996: F, t4283: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25284 = t6646 * t4292;
    let t25285 = t1888 * t25284;
    let t25287 = t4282 * t2647;
    let t25288 = t6646 * t25287;
    let t25289 = t22986 * t25288;
    let t25293 = t6547 * t7529;
    let t25299 = t25249 * t829;
    let t25300 = t6646 * t25299;
    let t25301 = t22986 * t25300;
    let t25303 = t22996 * t4283;
    (t25284, t25285, t25287, t25288, t25289, t25293, t25299, t25300, t25301, t25303)
}
