//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1154/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1154<F: Float>(t1647: F, t1907: F, t5706: F, t21085: F, t650: F, t653: F, t1853: F, t1883: F, t625: F, t5583: F, t626: F, t1906: F, t5381: F, t14: F, t178: F, t181: F, t188: F, t21066: F, t5670: F, t5673: F) -> (F, F, F, F, F, F) {
    let t21874 = 0.57895126195293126241e3 * t1907 * t5706 * t1647;
    let t21884 = 0.16081979498692535067e2 * t650 * t653 * t21085;
    let t21887 = 0.14246666666666666666e0 * t625 * t1853 * t1883;
    let t21899 = 0.71233333333333333332e-1 * t625 * t626 * t5583;
    let t21902 = 0.12372188467934141078e3 * t1906 * t1647 * t5381;
    let t21914 = 0.16053482475149032294e7 * t14 / t5670 / t181 * t188 / t5673 / t178 * t21066;
    (t21874, t21884, t21887, t21899, t21902, t21914)
}
