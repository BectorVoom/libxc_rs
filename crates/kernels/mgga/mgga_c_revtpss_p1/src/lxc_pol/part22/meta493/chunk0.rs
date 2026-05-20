//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2220/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2220<F: Float>(t15936: F, t16208: F, t1042: F, t3124: F, t4820: F, t1655: F, t697: F, t1011: F, t372: F, t4806: F, t15702: F, t15688: F, t3299: F) -> (F, F, F, F, F, F, F, F) {
    let t16209 = t16208 * t15936;
    let t16210 = t1042 * t16209;
    let t16218 = F::cast_from(0.28582678745379824648e-3_f64) * t3124 * t4820;
    let t16219 = t697 * t1655;
    let t16220 = t1011 * t16219;
    let t16222 = t372 * t4806;
    let t16223 = t16222 * t15702;
    let t16226 = t3299 * t15688;
    (t16209, t16210, t16218, t16219, t16220, t16222, t16223, t16226)
}
