//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3135/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3135<F: Float>(t12248: F, t16661: F, t3385: F, t12357: F, t1733: F, t3384: F, t12228: F, t12592: F, t5192: F, t1765: F, t45319: F, t12411: F, t17092: F) -> (F, F, F, F, F, F) {
    let t57802 = F::cast_from(0.28947563097646563121e3_f64) * t12248 * t16661 * t3385;
    let t57805 = F::new(2.0) * t3384 * t1733 * t12357;
    let t57808 = F::new(24.0) * t12248 * t1733 * t12228;
    let t57810 = F::cast_from(0.10254018858216406658e4_f64) * t5192 * t12592;
    let t57812 = F::cast_from(0.5848223622634646207e0_f64) * t45319 * t1765;
    let t57814 = F::new(6.0) * t17092 * t12411;
    (t57802, t57805, t57808, t57810, t57812, t57814)
}
