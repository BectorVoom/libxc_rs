//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1282/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1282<F: Float>(t1693: F, t1772: F, t7201: F, t1757: F, t1869: F, t33017: F, t62789: F, t34100: F, t5074: F, t33021: F, t5014: F, t2464: F, t36247: F, t4824: F, t7261: F, t654: F, t6973: F) -> (F, F, F, F, F, F, F) {
    let t116201 = t1693 * t7201 * t1772;
    let t116206 = t1869 * t33017 * t62789 * t1757;
    let t116210 = t5074 * t34100;
    let t116211 = 0.22109259259259259258e-2 * t116210;
    let t116212 = t5014 * t33021;
    let t116220 = t7261 * t36247 * t2464 * t4824;
    let t116223 = t6973 * t654;
    (t116201, t116206, t116210, t116211, t116212, t116220, t116223)
}
