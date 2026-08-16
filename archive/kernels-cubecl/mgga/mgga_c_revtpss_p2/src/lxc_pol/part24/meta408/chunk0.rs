//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1349/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1349<F: Float>(t123: F, t212: F, t9291: F, t10981: F, t588: F, t780: F, t39497: F, t787: F, t788: F, t10994: F, t2453: F, t39501: F, t781: F) -> (F, F, F, F, F) {
    let t40921 = t123 * t9291 * t212;
    let t40998 = F::cast_from(0.15709759505761725819e-2_f64) * t10981 * t780 * t588;
    let t41003 = F::cast_from(0.10118827226026589797e0_f64) * t787 * t788 * t39497;
    let t41011 = t2453 * t10994;
    let t41037 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t781;
    (t40921, t40998, t41003, t41011, t41037)
}
