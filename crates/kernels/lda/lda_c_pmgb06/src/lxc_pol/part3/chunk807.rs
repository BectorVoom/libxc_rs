//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 807/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk807<F: Float>(t4094: F, t591: F, t4096: F, t4111: F, t4103: F, t574: F, t581: F, t3050: F, t432: F, t1392: F, t1512: F, t3068: F, t161: F, t2886: F, t489: F, t1179: F, t4068: F, t573: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9422 = t4094 * t591;
    let t9424 = t4096 * t4111;
    let t9426 = t574 * t4103;
    let t9429 = 32.0 / 81.0 * t581 * t4103;
    let t9434 = t432 * t3050;
    let t9441 = t1512 * t1392;
    let t9443 = t432 * t3068;
    let t9450 = t161 * t489 * t2886;
    let t9457 = t573 * t1179 * t4068;
    (t9422, t9424, t9426, t9429, t9434, t9441, t9443, t9450, t9457)
}
