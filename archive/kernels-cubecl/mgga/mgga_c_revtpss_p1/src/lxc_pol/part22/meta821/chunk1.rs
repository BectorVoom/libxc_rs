//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2936/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2936<F: Float>(t10069: F, t14124: F, t14129: F, t14231: F, t10014: F, t14216: F, t13921: F, t4101: F, t686: F, t72: F, t10139: F, t136: F, t2457: F, t5659: F) -> (F, F, F, F, F, F) {
    let t47978 = t10069 * t14124;
    let t47980 = t10069 * t14129;
    let t47985 = t10069 * t14231;
    let t47995 = t10014 * t14216;
    let t47999 = t4101 * t13921 * t72 * t686;
    let t48003 = t10139 * t5659 * t136 * t2457;
    (t47978, t47980, t47985, t47995, t47999, t48003)
}
