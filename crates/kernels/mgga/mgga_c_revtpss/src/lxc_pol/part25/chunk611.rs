//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 611/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk611<F: Float>(t1211: F, t3568: F, t1209: F, t1269: F, t1214: F, t1294: F, t1277: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F) -> (F, F, F, F) {
    let t3569 = t1211 * t3568;
    let t3572 = t1209 * t1269;
    let t3575 = t1214 * t1294;
    let t3576 = t1277 * t3575;
    let t3579 = 0.19755555555555555556e-1 * t3356;
    let t3584 = t3579 - 0.9877777777777777778e-2 * t3358 - 0.9877777777777777778e-2 * t3365 + 0.29633333333333333334e-1 * t3370 + 0.14816666666666666667e-1 * t3374;
    (t3569, t3572, t3576, t3584)
}
