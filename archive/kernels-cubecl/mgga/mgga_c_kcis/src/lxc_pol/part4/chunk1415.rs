//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1415/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1415<F: Float>(t169: F, t1881: F, t3712: F, t1640: F, t5407: F, t446: F, t4505: F, t2132: F, t3708: F, t3709: F, t18376: F, t234: F, t441: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t18388 = t1881 * t3712;
    let t18390 = t5407 * t1640;
    let t18391 = t446 * t18390;
    let t18393 = t1881 * t4505;
    let t18395 = t3708 * t2132;
    let t18396 = t446 * t18395;
    let t18398 = t1881 * t3709;
    let t18401 = piecewise3::<F>(t170, F::cast_from(0.0_f64), -t18376);
    let t18402 = t234 * t18401;
    let t18403 = t18402 * t441;
    (t18388, t18391, t18393, t18396, t18398, t18403)
}
