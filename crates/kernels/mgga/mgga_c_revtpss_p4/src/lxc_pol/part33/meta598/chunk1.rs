//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2020/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2020<F: Float>(t94590: F, t94771: F, t25304: F, t25949: F, t25946: F, t25878: F, t94661: F, t7246: F, t9692: F, t1419: F, t7063: F, t25898: F) -> (F, F, F, F, F, F) {
    let t94772 = t94771 * t94590;
    let t94776 = t25304 * t25949;
    let t94777 = t94776 * t25946;
    let t94779 = t25878 * t94661;
    let t94784 = F::cast_from(0.30356481678079769392e-1_f64) * t7246 * t9692;
    let t94801 = t7063 * t1419;
    let t94802 = t94801 * t25898;
    (t94772, t94777, t94779, t94784, t94801, t94802)
}
