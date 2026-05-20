//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1997/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1997<F: Float>(t25310: F, t25331: F, t2435: F, t25339: F, t11064: F, t7086: F, t25604: F, t995: F, t357: F, t988: F, t355: F, t1071: F, t11239: F) -> (F, F, F, F, F, F) {
    let t93384 = t25310 * t25331;
    let t93391 = t2435 * t25339;
    let t93404 = t7086 * t11064;
    let t93436 = t995 * t25604;
    let t93437 = t357 * t988;
    let t93438 = t355 * t93437;
    let t93488 = t1071 * t11239;
    (t93384, t93391, t93404, t93436, t93438, t93488)
}
