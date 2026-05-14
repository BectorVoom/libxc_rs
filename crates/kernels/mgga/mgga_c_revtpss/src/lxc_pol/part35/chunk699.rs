//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 699/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk699<F: Float>(t1386: F, t820: F, t844: F, t2482: F, t596: F, t1384: F, t235: F, t239: F, t4003: F, t543: F, t27: F, t4000: F, t555: F, t5744: F, t786: F, t4083: F, t9303: F) -> (F, F, F, F, F, F, F, F) {
    let t9962 = t820 * t1386 * t844;
    let t9976 = t2482 * t1386 * t596;
    let t9989 = t1384 * t1384;
    let t9990 = 1.0 / t9989;
    let t9991 = t9990 * t235;
    let t9993 = t820 * t9991 * t239;
    let t9994 = t4003 * t543;
    let t10001 = t2482 * t4000 * t27;
    let t10022 = t5744 * t555;
    let t10023 = t786 * t10022;
    let t10035 = 0.26019841438354088051e-2 * t9303 * t4083;
    (t9962, t9976, t9990, t9993, t9994, t10001, t10023, t10035)
}
