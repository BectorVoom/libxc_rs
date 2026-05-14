//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 894/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk894<F: Float>(t125: F, t886: F, t246: F, t244: F, t31838: F, t239: F, t2718: F, t8484: F, t8478: F) -> (F, F, F, F, F, F) {
    let t31839 = t125 * t886;
    let t31840 = t246 * t31839;
    let t31841 = t244 * t31840;
    let t31842 = t31838 * t31841;
    let t31844 = t2718 * t239;
    let t31845 = t8484 * t31844;
    let t31846 = t8478 * t31845;
    (t31840, t31841, t31842, t31844, t31845, t31846)
}
