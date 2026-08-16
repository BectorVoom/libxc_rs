//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 272/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk272<F: Float>(t158: F, t750: F, t162: F, t716: F, t187: F, t192: F, t72: F, t186: F, t675: F, t685: F) -> (F, F, F, F, F) {
    let t751 = t158 * t750;
    let t752 = t716 * t162;
    let t754 = F::cast_from(0.19751673498613801407e-1_f64) * t752 * t187;
    let t755 = t192 * t72;
    let t757 = t685 * t675 * t186;
    (t751, t752, t754, t755, t757)
}
