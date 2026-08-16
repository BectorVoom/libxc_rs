//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 992/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk992<F: Float>(t30350: F, t30379: F, t1254: F, t13682: F, t30318: F, t1275: F, t7993: F, t6100: F, t2141: F, t7976: F, t4126: F, t13561: F) -> (F, F, F, F, F, F) {
    let t30380 = t30350 + t30379;
    let t30381 = t30380 * t1254;
    let t30384 = t30318 * t13682;
    let t30387 = t1275 * t7993;
    let t30388 = t6100 * t30387;
    let t30391 = t7976 * t2141;
    let t30393 = t4126 * t30391 * t1275;
    let t30396 = t13561 * t30391;
    (t30381, t30384, t30388, t30391, t30393, t30396)
}
