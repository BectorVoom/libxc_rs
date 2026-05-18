//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 717/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk717<F: Float>(t2035: F, t7235: F, t2033: F, t531: F, t1353: F, t1450: F, t2014: F, t2022: F, t212: F, t1358: F, t689: F, t2023: F, t786: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7236 = t7235 * t2035;
    let t7237 = t531 * t2033;
    let t7238 = t1450 * t1353;
    let t7239 = t7237 * t7238;
    let t7241 = F::new(3.0) * t2014 * t7239;
    let t7242 = t212 * t2022;
    let t7243 = t7242 * t1358;
    let t7245 = F::new(0.54878743191129263322e-2) * t689 * t7243;
    let t7246 = t786 * t2023;
    (t7236, t7237, t7238, t7239, t7241, t7242, t7243, t7245, t7246)
}
