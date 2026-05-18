//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1334/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1334<F: Float>(t26054: F, t9671: F, t1419: F, t7063: F, t25898: F, t25901: F, t136: F, t2457: F, t7307: F, t25944: F, t26035: F, t686: F, t72: F) -> (F, F, F, F, F, F) {
    let t94799 = t26054 * t9671;
    let t94801 = t7063 * t1419;
    let t94802 = t94801 * t25898;
    let t94803 = t94802 * t25901;
    let t94806 = t7307 * t136 * t2457;
    let t94807 = t25944 * t94806;
    let t94810 = t26035 * t72 * t686;
    (t94799, t94801, t94803, t94806, t94807, t94810)
}
