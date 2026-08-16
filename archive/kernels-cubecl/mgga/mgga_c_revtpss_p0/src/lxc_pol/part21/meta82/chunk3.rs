//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 592/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk592<F: Float>(t1163: F, t1166: F, t1717: F, t1724: F, t1727: F, t1730: F) -> F {
    let t1744 = F::cast_from(0.3529725e1_f64) * t1724 - t1163 + F::cast_from(0.516475e0_f64) * t1717 + F::cast_from(0.6311625e0_f64) * t1727 - t1166 + F::cast_from(0.104195e0_f64) * t1730;
    t1744
}
