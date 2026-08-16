//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2041/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2041<F: Float>(t1904: F, t25912: F, t689: F, t1385: F, t7910: F, t14104: F, t94725: F, t1358: F, t2439: F, t785: F, t2435: F, t7925: F) -> (F, F, F, F, F) {
    let t97869 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t25912 * t1904;
    let t97875 = t1385 * t7910;
    let t97882 = t94725 * t14104;
    let t97894 = t2439 * t785 * t7910 * t1358;
    let t97899 = t7925 * t2435;
    (t97869, t97875, t97882, t97894, t97899)
}
