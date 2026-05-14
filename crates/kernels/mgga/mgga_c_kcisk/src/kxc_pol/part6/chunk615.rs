//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 615/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk615<F: Float>(t776: F, t2442: F, t2620: F, t525: F, t642: F, t773: F, t8781: F, t8787: F, t9192: F, t79: F, t781: F, t2063: F, t2642: F, t5491: F, t1775: F, t5497: F, t7715: F) -> (F, F, F, F, F) {
    let t777 = t776 < -0.66725e-1;
    let t9206 = piecewise3(t777, 0.0, 10.0 / 9.0 * t525 * t9192 * t642 - 20.0 / 27.0 * t525 * t2620 * t2442 + 40.0 / 81.0 * t525 * t773 * t8781 - 10.0 / 27.0 * t525 * t773 * t8787);
    let t9207 = t79 * t9206;
    let t9208 = t9207 * t781;
    let t9212 = t2063 * t2642;
    let t9213 = t5491 * t9212;
    let t9214 = t1775 * t9213;
    let t9217 = t5497 * t7715;
    (t9207, t9208, t9213, t9214, t9217)
}
