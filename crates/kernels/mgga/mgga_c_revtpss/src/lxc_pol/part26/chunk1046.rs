//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1046/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1046<F: Float>(t2408: F, t30: F, t605: F, t890: F, t2832: F, t2394: F, t33: F, t2411: F, t14365: F, t1113: F, t775: F, t2430: F) -> (F, F, F, F, F, F, F, F) {
    let t25446 = t30 * t2408;
    let t25449 = t605 * t890;
    let t25452 = t30 * t2832;
    let t25752 = t33 * t2394;
    let t25759 = t2411 * t33;
    let t25760 = t25759 * t14365;
    let t25763 = t1113 * t775;
    let t25767 = t33 * t2430;
    (t25446, t25449, t25452, t25752, t25759, t25760, t25763, t25767)
}
