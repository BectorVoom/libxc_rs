//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1042/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1042<F: Float>(t14815: F, t376: F, t1170: F, t3474: F, t5053: F, t1809: F, t3448: F, t10745: F, t5099: F, t1797: F, t3343: F, t3348: F, t13321: F, t381: F, t3444: F, t10513: F, t284: F) -> (F, F, F, F, F, F, F, F) {
    let t14816 = t376 * t14815;
    let t14817 = t1170 * t14816;
    let t14819 = t3474 * t5053;
    let t14821 = t1809 * t3448;
    let t14823 = t10745 * t5099;
    let t14825 = t1797 * t3343;
    let t14827 = t1809 * t3348;
    let t14829 = t13321 * t381;
    let t14830 = t14829 * t3444;
    let t14832 = t10513 * t284;
    (t14817, t14819, t14821, t14823, t14825, t14827, t14830, t14832)
}
