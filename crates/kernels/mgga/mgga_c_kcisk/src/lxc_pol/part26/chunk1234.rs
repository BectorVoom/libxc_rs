//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1234/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1234<F: Float>(t3185: F, t9335: F, t3187: F, t10336: F, t9358: F, t1001: F, t3138: F, t3174: F, t979: F, t32592: F, t32672: F, t1123: F, t15497: F, t32589: F, t9365: F, t1128: F, t3376: F, t3417: F) -> (F, F, F, F, F, F, F) {
    let t110905 = t9335 * t3185;
    let t110907 = 6.0 * t110905 * t3187;
    let t110912 = 18.0 * t10336 * t9358 * t3187;
    let t110920 = t979 * t3138 * t3174 * t1001;
    let t110922 = t32672 * t32592;
    let t110925 = t32589 * t15497 * t1123;
    let t110926 = t9365 * t110925;
    let t110930 = t32589 * t3376 * t1128 * t3417;
    (t110907, t110912, t110920, t110922, t110925, t110926, t110930)
}
