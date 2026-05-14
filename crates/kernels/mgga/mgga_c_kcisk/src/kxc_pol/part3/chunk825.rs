//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 825/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk825<F: Float>(t13561: F, t13562: F, t13565: F, t1273: F, t4100: F, t1275: F, t4120: F, t1233: F, t4023: F, t4126: F, t1229: F, t4080: F, t357: F, t4079: F, t346: F, t1253: F, t4032: F) -> (F, F, F, F, F, F, F) {
    let t13566 = t13561 * t13562 * t13565;
    let t13569 = t4100 * t1273;
    let t13570 = t1275 * t4120;
    let t13571 = t13569 * t13570;
    let t13574 = t4023 * t1233;
    let t13578 = t4126 * t13562 * t1275;
    let t13583 = t1229 * t4080;
    let t13587 = 1.0 / t4079 / t357;
    let t13588 = t346 * t13587;
    let t13589 = t4032 * t1253;
    (t13566, t13571, t13574, t13578, t13583, t13588, t13589)
}
