//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3044/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3044<F: Float>(t25026: F, t3801: F, t1187: F, t1756: F, t58672: F, t69511: F, t1130: F, t24466: F, t1151: F, t58339: F, t6439: F, t12243: F, t24221: F) -> (F, F, F, F, F) {
    let t81139 = t25026 * t3801;
    let t81145 = F::cast_from(0.30762056574649219973e4_f64) * t58672 * t69511 * t1756 * t1187;
    let t81146 = t24466 * t1130;
    let t81148 = F::cast_from(1.0_f64) * t81146 * t1151;
    let t81150 = F::cast_from(6.0_f64) * t58339 * t6439;
    let t81152 = F::cast_from(6.0_f64) * t12243 * t24221;
    (t81139, t81145, t81148, t81150, t81152)
}
