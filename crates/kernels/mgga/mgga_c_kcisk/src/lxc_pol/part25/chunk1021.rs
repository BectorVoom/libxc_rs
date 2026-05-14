//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1021/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1021<F: Float>(t16562: F, t196: F, t1646: F, t1909: F, t11: F, t139: F, t41: F, t7379: F, t7371: F, t18000: F, t2488: F, t4663: F, t725: F, t1860: F, t4594: F, t6759: F) -> (F, F, F, F, F, F, F) {
    let t18042 = t16562 * t196;
    let t18045 = t1909 * t1646;
    let t18053 = t139 * t11 * t41;
    let t18054 = t18053 * t7379;
    let t18057 = 0.5895802469135802469e-1 * t18053 * t7371;
    let t18058 = t2488 * t18000;
    let t18063 = t725 * t4663;
    let t18069 = t4594 * t1860;
    let t18070 = t18069 * t6759;
    (t18042, t18045, t18054, t18057, t18058, t18063, t18070)
}
