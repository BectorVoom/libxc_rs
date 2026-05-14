//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 936/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk936<F: Float>(t121346: F, t119971: F, t32237: F, t121142: F, t1412: F, t844: F, t32291: F, t8591: F, t121166: F, t25304: F, t8571: F, t121035: F, t32268: F, t121134: F, t32296: F, t531: F) -> (F, F, F, F, F, F, F, F) {
    let t121347 = 0.18822977838986977999e-5 * t121346;
    let t121348 = t119971 * t32237;
    let t121350 = 0.6019057092162847523e-2 * t121348 * t121142;
    let t121354 = t844 * t1412;
    let t121356 = t8591 * t121354 * t32291;
    let t121363 = t25304 * t8571 * t121166;
    let t121364 = 0.17851433602423232928e-4 * t121363;
    let t121365 = t32268 * t121035;
    let t121366 = t121365 * t121134;
    let t121441 = t531 * t32296;
    (t121347, t121350, t121354, t121356, t121364, t121365, t121366, t121441)
}
