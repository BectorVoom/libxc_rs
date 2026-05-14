//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1217/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1217<F: Float>(t9442: F, t9792: F, t1413: F, t2212: F, t1441: F, t415: F, t9474: F, t1333: F, t9818: F, t32045: F, t6234: F, t1339: F, t5968: F, t9469: F, t32066: F, t32155: F, t32157: F, t32174: F, t32177: F, t33373: F, t9449: F, t9796: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33555 = t9792 * t9442;
    let t33557 = t2212 * t1413;
    let t33558 = t33557 * t1441;
    let t33559 = t415 * t33558;
    let t33561 = t2212 * t9474;
    let t33562 = t415 * t33561;
    let t33564 = t1333 * t9818;
    let t33570 = t32045 * t6234;
    let t33571 = t1339 * t33570;
    let t33577 = t9469 * t5968;
    let t33578 = t415 * t33577;
    let t33580 = -0.34722222222222222223e-2 * t33555 - 0.24872916666666666666e-2 * t33559 - 0.66327777777777777776e-2 * t33562 + 0.16581944444444444444e-2 * t33564 - 0.16581944444444444444e-2 * t32155 + 0.11054629629629629629e-2 * t32157 + 0.40208333333333333335e-2 * t32066 * t9796 - 0.16581944444444444444e-2 * t33571 - 0.34722222222222222223e-2 * t33373 * t9449 + 0.34722222222222222223e-2 * t32174 + 0.34722222222222222223e-2 * t32177 - 0.24872916666666666666e-2 * t33578;
    (t33557, t33558, t33559, t33561, t33562, t33564, t33570, t33571, t33577, t33578, t33580)
}
