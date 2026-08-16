//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2080/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2080<F: Float>(t14923: F, t4368: F, t2724: F, t4364: F, t4365: F, t2482: F, t2719: F, t814: F) -> (F, F, F) {
    let t14925 = F::cast_from(0.40015750243531754508e-2_f64) * t14923 * t4368;
    let t14927 = t4364 * t4365 * t2724;
    let t14931 = t2482 * t2719 * t814;
    (t14925, t14927, t14931)
}
