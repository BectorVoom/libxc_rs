//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 867/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk867<F: Float>(t222: F, t294: F, t30174: F, t30158: F, t295: F, t559: F, t2231: F, t7706: F, t5625: F, t3796: F, t3482: F, t2152: F, t14255: F, t3484: F, t5633: F, t5606: F, t8082: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t30175 = t294 * t30174;
    let t30176 = 3.0 / 16.0 * t30175;
    let t30177 = piecewise3(t223, 0.0, t30158);
    let t30178 = t295 * t30177;
    let t30179 = t30178 * t559;
    let t30180 = t294 * t30179;
    let t30181 = t30180 / 16.0;
    let t30184 = t7706 * t2231;
    let t30185 = t5625 * t30184;
    let t30186 = t3796 * t30185;
    let t30187 = t3482 * t30186;
    let t30189 = t7706 * t2152;
    let t30190 = t14255 * t30189;
    let t30191 = t3484 * t30190;
    let t30192 = t5633 * t30191;
    let t30194 = t5606 * t8082;
    (t30176, t30181, t30184, t30187, t30189, t30192, t30194)
}
