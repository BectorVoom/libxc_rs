//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2269/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2269<F: Float>(t111690: F, t111704: F, t111717: F, t111746: F, t111762: F, t111770: F, t111796: F, t113012: F, t2172: F, t6936: F, t1921: F, t8240: F) -> (F, F, F) {
    let t113015 = t111690 + t111704 + t111717 + t111746 + t111762 + t111770 + t111796 + t113012;
    let t113019 = t6936 * t2172;
    let t113022 = t8240 * t1921;
    (t113015, t113019, t113022)
}
