//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2807/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2807<F: Float>(t10073: F, t10929: F, t10069: F, t10654: F, t2790: F, t9292: F, t11003: F, t9303: F, t10981: F, t22: F, t868: F, t886: F) -> (F, F, F, F, F) {
    let t40954 = t10073 * t10929;
    let t40956 = t10069 * t10654;
    let t40958 = t9292 * t2790;
    let t40970 = t9303 * t11003;
    let t40978 = t10981 * t868 * t22 * t886;
    (t40954, t40956, t40958, t40970, t40978)
}
