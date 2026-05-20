//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1032/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1032<F: Float>(t2430: F, t30: F, t1946: F, t2684: F, t7043: F, t820: F, t843: F, t857: F, t2656: F, t7045: F, t240: F, t7036: F) -> (F, F, F, F, F, F) {
    let t25215 = t30 * t2430;
    let t25219 = t1946 * t2684;
    let t25222 = t820 * t7043 * t843;
    let t25223 = t25222 * t857;
    let t25225 = t7045 * t2656;
    let t25227 = t7036 * t240;
    (t25215, t25219, t25222, t25223, t25225, t25227)
}
