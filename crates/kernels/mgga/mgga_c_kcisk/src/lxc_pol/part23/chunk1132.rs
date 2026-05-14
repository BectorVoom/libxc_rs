//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1132/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1132<F: Float>(t1322: F, t1327: F, t32069: F, t6204: F, t3791: F, t9469: F, t415: F, t3495: F, t468: F, t3503: F, t3491: F, t9433: F, t21499: F, t9445: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32070 = t1327 * t1322;
    let t32071 = t32069 * t32070;
    let t32072 = t6204 * t32071;
    let t32075 = t9469 * t3791;
    let t32076 = t415 * t32075;
    let t32078 = t468 * t3495;
    let t32079 = t415 * t32078;
    let t32081 = t468 * t3503;
    let t32082 = t415 * t32081;
    let t32084 = t3491 * t9433;
    let t32087 = t9445 * t21499;
    (t32070, t32071, t32072, t32075, t32076, t32078, t32079, t32081, t32082, t32084, t32087)
}
