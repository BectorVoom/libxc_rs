//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1299/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1299<F: Float>(t1020: F, t7719: F, t95893: F, t14563: F, t5329: F, t7691: F, t27876: F, t2822: F, t4792: F, t92701: F, t13186: F, t26760: F) -> (F, F, F, F, F) {
    let t95895 = t1020 * t95893 * t7719;
    let t95898 = t5329 * t7691 * t14563;
    let t95903 = t2822 * t27876;
    let t95906 = t1020 * t92701 * t4792;
    let t95909 = t1020 * t26760 * t13186;
    (t95895, t95898, t95903, t95906, t95909)
}
