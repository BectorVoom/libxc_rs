//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 895/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk895<F: Float>(t14874: F, t1172: F, t10525: F, t284: F, t5048: F, t1175: F, t5042: F, t1170: F, t1143: F, t346: F, t932: F, t14051: F, t143: F, t1780: F, t245: F, t3393: F, t5155: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t14875 = t14874 * sigma0;
    let t14876 = t14875 * t1172;
    let t14878 = t10525 * t284;
    let t14879 = t14878 * t5048;
    let t14881 = t1175 * t5042;
    let t14882 = t1170 * t14881;
    let t14896 = t1143 * t346;
    let t14899 = t1143 * t932;
    let t14902 = t14051 * t143;
    let t14907 = t1780 * t245;
    let t14913 = t3393 * t5155;
    (t14876, t14879, t14882, t14896, t14899, t14902, t14907, t14913)
}
