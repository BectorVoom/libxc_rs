//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1165/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1165<F: Float>(t27619: F, t7150: F, t3268: F, t373: F, t31991: F, t99914: F, t1678: F, t31902: F, t127: F, t31950: F, t33825: F, t371: F) -> (F, F, F, F, F) {
    let t126749 = t7150 * t27619;
    let t126765 = t373 * t3268;
    let t126770 = t99914 * t31991;
    let t126774 = t31902 * t1678 * t31991;
    let t126779 = t31950 * t371 * t127 * t33825;
    (t126749, t126765, t126770, t126774, t126779)
}
