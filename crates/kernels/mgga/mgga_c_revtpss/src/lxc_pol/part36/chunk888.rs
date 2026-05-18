//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 888/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk888<F: Float>(t1719: F, t3432: F, t1729: F, t2439: F, t1737: F, t3451: F, t3476: F, t3383: F, t1749: F, t3520: F, t3495: F, t1770: F, t3781: F) -> (F, F, F, F, F, F, F, F) {
    let t16840 = t1719 * t3432;
    let t16876 = t2439 * t1729;
    let t17023 = t1737 * t3451;
    let t17032 = t1737 * t3476;
    let t17092 = t1719 * t3383;
    let t17097 = t1749 * t3520;
    let t17154 = t1749 * t3495;
    let t17183 = t1770 * t3781;
    (t16840, t16876, t17023, t17032, t17092, t17097, t17154, t17183)
}
