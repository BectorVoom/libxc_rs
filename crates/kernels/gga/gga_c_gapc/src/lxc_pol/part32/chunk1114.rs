//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1114/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1114<F: Float>(t11679: F, t24092: F, t6182: F, t9497: F, t10346: F, t134: F, t6939: F, t11210: F, t11657: F, t7557: F, t11662: F, t2200: F, t6857: F, t2221: F, t3729: F, t11661: F, t23609: F, t23612: F, t829: F) -> (F, F, F, F, F, F, F) {
    let t35890 = t11679 * t24092;
    let t35894 = t6182 * t9497;
    let t35895 = t10346 * t6939 * t134 * t35894;
    let t35898 = t11657 * t11210 * t7557;
    let t35901 = t11662 * t2200 * t6857;
    let t35903 = t2221 * t3729;
    let t35907 = t11661 * t23609 * t829 * t23612;
    (t35890, t35894, t35895, t35898, t35901, t35903, t35907)
}
