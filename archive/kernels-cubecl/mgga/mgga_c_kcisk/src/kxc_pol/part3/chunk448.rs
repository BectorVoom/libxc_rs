//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 448/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk448<F: Float>(t1425: F, t3521: F, t1417: F, t1430: F, t1435: F, t313: F, t442: F) -> (F, F, F, F) {
    let t3522 = t3521 * t1425;
    let t3524 = t1417 * t1430;
    let t3526 = t1417 * t1435;
    let t3528 = t313 * t442;
    let t3529 = F::cast_from(1.0_f64) / t3528;
    (t3522, t3524, t3526, t3529)
}
