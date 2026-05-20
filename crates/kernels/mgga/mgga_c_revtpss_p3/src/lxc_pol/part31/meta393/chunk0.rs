//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1434/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1434<F: Float>(t1160: F, t5117: F, t1737: F, t3476: F, t16868: F, t16712: F, t16892: F, t16708: F, t1179: F, t5155: F, t1719: F, t3383: F) -> (F, F, F, F, F, F, F, F) {
    let t17026 = t5117 * t1160;
    let t17032 = t1737 * t3476;
    let t17050 = F::cast_from(0.13892666666666666667e0_f64) * t16868;
    let t17052 = F::cast_from(0.34431666666666666666e0_f64) * t16712;
    let t17066 = F::cast_from(0.27785333333333333334e0_f64) * t16892;
    let t17075 = F::cast_from(0.22954444444444444444e0_f64) * t16708;
    let t17089 = t5155 * t1179;
    let t17092 = t1719 * t3383;
    (t17026, t17032, t17050, t17052, t17066, t17075, t17089, t17092)
}
