//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 848/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk848<F: Float>(t233: F, t867: F, t1949: F, t7056: F, t10073: F, t1957: F, t822: F, t25386: F, t676: F, t837: F, t25377: F, t2718: F) -> (F, F, F, F, F, F, F) {
    let t25402 = t867 * t233;
    let t25403 = t25402 * t1949;
    let t25404 = t7056 * t25403;
    let t25406 = F::new(0.24093411633903331839e-3) * t10073 * t25404;
    let t25410 = t1957 * t822;
    let t25411 = t25386 * t25410;
    let t25412 = t676 * t837;
    let t25413 = t25377 * t25412;
    let t25414 = t25411 * t25413;
    let t25416 = t867 * t2718;
    (t25406, t25410, t25411, t25412, t25413, t25414, t25416)
}
