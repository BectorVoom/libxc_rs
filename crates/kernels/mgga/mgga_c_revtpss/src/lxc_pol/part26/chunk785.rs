//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 785/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk785<F: Float>(t1398: F, t1419: F, t4086: F, t543: F, t2782: F, t4056: F, t555: F, t9990: F, t1432: F, t2470: F, t4107: F, t1433: F, t9288: F) -> (F, F, F, F, F, F) {
    let t10079 = t4086 * t1419 * t1398 * t543;
    let t10080 = t2782 * t10079;
    let t10082 = t555 * t4056;
    let t10084 = t4086 * t10082 * t543;
    let t10085 = t2782 * t10084;
    let t10090 = t9990 * t555;
    let t10098 = t1432 * t4107 * t2470;
    let t10102 = F::cast_from(0.30356481678079769392e-1_f64) * t1432 * t1433 * t9288;
    (t10080, t10082, t10085, t10090, t10098, t10102)
}
