//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 927/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk927<F: Float>(t1384: F, t235: F, t4003: F, t543: F, t2482: F, t27: F, t4000: F, t1419: F, t4086: F, t786: F, t555: F, t5744: F, t2435: F, t4093: F, t4083: F, t9303: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9989 = t1384 * t1384;
    let t9990 = 1.0 / t9989;
    let t9991 = t9990 * t235;
    let t9994 = t4003 * t543;
    let t10001 = t2482 * t4000 * t27;
    let t10013 = t4086 * t1419;
    let t10014 = t786 * t10013;
    let t10022 = t5744 * t555;
    let t10023 = t786 * t10022;
    let t10032 = t2435 * t4093;
    let t10035 = 0.26019841438354088051e-2 * t9303 * t4083;
    (t9990, t9991, t9994, t10001, t10014, t10022, t10023, t10032, t10035)
}
