//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2070/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2070<F: Float>(t25946: F, t94776: F, t25878: F, t94661: F, t7246: F, t9692: F, t26054: F, t9671: F, t1419: F, t7063: F, t25898: F, t25901: F) -> (F, F, F, F, F, F, F) {
    let t94777 = t94776 * t25946;
    let t94779 = t25878 * t94661;
    let t94784 = F::cast_from(0.30356481678079769392e-1_f64) * t7246 * t9692;
    let t94799 = t26054 * t9671;
    let t94801 = t7063 * t1419;
    let t94802 = t94801 * t25898;
    let t94803 = t94802 * t25901;
    (t94777, t94779, t94784, t94799, t94801, t94802, t94803)
}
