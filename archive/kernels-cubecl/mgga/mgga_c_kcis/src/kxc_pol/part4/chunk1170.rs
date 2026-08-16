//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1170/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1170<F: Float>(t1797: F, t3343: F, t1809: F, t3348: F, t13321: F, t381: F, t3444: F, t10513: F, t284: F, t14616: F, t5047: F, t10753: F, t5099: F) -> (F, F, F, F, F) {
    let t14825 = t1797 * t3343;
    let t14827 = t1809 * t3348;
    let t14829 = t13321 * t381;
    let t14830 = t14829 * t3444;
    let t14832 = t10513 * t284;
    let t14833 = t5047 * t14616;
    let t14834 = t14832 * t14833;
    let t14836 = t10753 * t5099;
    (t14825, t14827, t14830, t14834, t14836)
}
