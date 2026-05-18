//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 887/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk887<F: Float>(t13484: F, t13539: F, t13576: F, t13616: F, t4852: F, t829: F, t1728: F, t2635: F, t3073: F, t4670: F, t1045: F, t3096: F, t4848: F) -> (F, F, F, F, F) {
    let t13618 = t13484 + t13539 + t13576 + t13616;
    let t13620 = t4852 * t829;
    let t13623 = t1728 * t2635;
    let t13626 = t3073 * t4670;
    let t13627 = t13626 * t1045;
    let t13630 = t4848 * t3096;
    (t13618, t13620, t13623, t13627, t13630)
}
