//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2878/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2878<F: Float>(t11064: F, t23429: F, t18268: F, t2403: F, t39756: F, t39760: F, t39764: F, t39770: F, t39773: F, t4343: F, t49930: F, t76967: F, t76969: F, t76970: F, t76973: F) -> (F, F) {
    let t77373 = t23429 * t11064;
    let t77381 = -F::cast_from(9.0_f64) * t18268 * t2403 * t4343 + t39756 + t39760 - t39764 + t39770 + t39773 + t49930 - t76967 + t76969 + t76970 - t76973;
    (t77373, t77381)
}
