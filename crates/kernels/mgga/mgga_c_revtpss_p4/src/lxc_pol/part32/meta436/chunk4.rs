//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1577/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1577<F: Float>(t21512: F, t5480: F, t1280: F, t20747: F, t5230: F, t5486: F, t21342: F, t489: F, t1248: F, t1287: F, t6695: F, t1774: F, t17821: F) -> (F, F, F, F, F, F) {
    let t21518 = t21512 * t5480;
    let t21521 = t1280 * t20747;
    let t21524 = t5486 * t5230;
    let t21527 = t489 * t21342;
    let t21535 = t6695 * t1248 * t1287;
    let t21538 = t17821 * t1774;
    (t21518, t21521, t21524, t21527, t21535, t21538)
}
