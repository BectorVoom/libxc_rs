//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3202/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3202<F: Float>(t45786: F, t58919: F, t17708: F, t45846: F, t12975: F, t1803: F, t225: F, t56412: F, t480: F, t12984: F, t5323: F, t12916: F, t17390: F, t3718: F) -> (F, F, F, F, F, F, F) {
    let t59011 = t45786 * t58919;
    let t59017 = t45846 * t17708;
    let t59025 = t12975 * t1803;
    let t59032 = t56412 * t225;
    let t59033 = t59032 * t480;
    let t59040 = t5323 * t12984;
    let t59041 = F::cast_from(0.7622047665434619906e-3_f64) * t59040;
    let t59043 = t3718 * t12916 * t17390;
    (t59011, t59017, t59025, t59032, t59033, t59041, t59043)
}
