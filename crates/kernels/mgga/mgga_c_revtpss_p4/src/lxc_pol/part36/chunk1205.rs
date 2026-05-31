//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1205/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1205<F: Float>(t2163: F, t5920: F, t1518: F, t29427: F, t30137: F, t30140: F, t30142: F, t30145: F, t30147: F, t30149: F, t30716: F, t30724: F, t7586: F) -> (F, F) {
    let t30951 = t2163 * t5920;
    let t30959 = F::cast_from(4.0_f64) * t1518 * t29427 + F::cast_from(2.0_f64) * t5920 * t7586 + t30137 + t30140 + t30142 + t30145 + t30147 + t30149 + t30716 + F::cast_from(2.0_f64) * t30724;
    (t30951, t30959)
}
