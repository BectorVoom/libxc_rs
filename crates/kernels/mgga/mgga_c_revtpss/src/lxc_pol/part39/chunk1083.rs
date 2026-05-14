//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1083/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1083<F: Float>(t13847: F, t13848: F, t1399: F, t9816: F, t2713: F, t3964: F, t5617: F, t1872: F, t3829: F, t800: F, t124: F, t13716: F, t5686: F, t9744: F, t1353: F, t5689: F) -> (F, F, F, F, F, F) {
    let t14005 = t13847 * t13848 * t1399;
    let t14007 = 0.25410001404642664112e-4 * t9816 * t14005;
    let t14013 = t3964 * t2713 * t5617;
    let t14016 = t800 * t1872 * t3829;
    let t14019 = t124 * t13716;
    let t14020 = t800 * t14019;
    let t14024 = 7.0 / 24.0 * t9744 * t5686;
    let t14026 = t800 * t5689 * t1353;
    (t14007, t14013, t14016, t14020, t14024, t14026)
}
