//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2912/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2912<F: Float>(t245: F, t47247: F, t2713: F, t3964: F, t9714: F, t3951: F, t9732: F, t136: F, t4010: F, t220: F, t9905: F, t9976: F) -> (F, F, F, F, F, F) {
    let t47248 = t47247 * t245;
    let t47259 = t3964 * t2713 * t9714;
    let t47262 = t3964 * t9732 * t3951;
    let t47273 = t4010 * t136;
    let t47274 = t47273 * t220;
    let t47298 = t9976 * t9905;
    (t47248, t47259, t47262, t47273, t47274, t47298)
}
