//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 987/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk987<F: Float>(t1170: F, t14881: F, t1143: F, t346: F, t932: F, t14051: F, t143: F, t1780: F, t245: F, t3393: F, t5155: F, t330: F, t4920: F) -> (F, F, F, F, F, F, F) {
    let t14882 = t1170 * t14881;
    let t14896 = t1143 * t346;
    let t14899 = t1143 * t932;
    let t14902 = t14051 * t143;
    let t14907 = t1780 * t245;
    let t14913 = t3393 * t5155;
    let t14915 = t4920 * t330;
    (t14882, t14896, t14899, t14902, t14907, t14913, t14915)
}
