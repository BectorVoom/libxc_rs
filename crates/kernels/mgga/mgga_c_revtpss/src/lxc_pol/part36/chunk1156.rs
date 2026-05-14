//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1156/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1156<F: Float>(t1032: F, t6695: F, t2148: F, t1209: F, t30882: F, t7658: F, t7642: F, t5219: F, t8190: F, t7635: F, t30923: F, t3801: F, t2172: F, t6936: F, t1921: F, t8240: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112757 = t6695 * t1032;
    let t112758 = t2148 * t112757;
    let t112774 = t1209 * t112757;
    let t112843 = t30882 * t7658;
    let t112880 = t7642 * t112757;
    let t112902 = t5219 * t8190;
    let t112943 = t30882 * t7635;
    let t112958 = t30923 * t3801;
    let t113019 = t6936 * t2172;
    let t113022 = t8240 * t1921;
    (t112758, t112774, t112843, t112880, t112902, t112943, t112958, t113019, t113022)
}
