//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 744/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk744<F: Float>(t1495: F, t4161: F, t1360: F, t3960: F, t1460: F, t3245: F, t10470: F, t558: F, t1014: F, t4111: F, t1385: F, t3717: F, t530: F, t64: F, t555: F, t491: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12281 = t4161 * t1495;
    let t12286 = t1360 * t3960;
    let t12303 = t3245 * t1460;
    let t12305 = t10470 * t558;
    let t12306 = 0.73697530864197530862e-3 * t12305;
    let t12307 = t1014 * t4111;
    let t12309 = t1385 * t3717;
    let t12319 = t64 * t530;
    let t12321 = 1.0 / t555 / t12319;
    let t12322 = t491 * t12321;
    (t12281, t12286, t12303, t12305, t12306, t12307, t12309, t12321, t12322)
}
