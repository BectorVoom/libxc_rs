//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1227/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1227<F: Float>(t34346: F, t34348: F, t34350: F, t34352: F, t34354: F, t34356: F, t34358: F, t34360: F, t34362: F, t34364: F, t34366: F, t34369: F, t34371: F, t34635: F, t10028: F, t10039: F, t12345: F, t12352: F, t2042: F, t34289: F, t34292: F, t34297: F, t34302: F, t34305: F, t34375: F, t34386: F, t34608: F, t34612: F, t34615: F, t34618: F, t5527: F, t5532: F, t7690: F, t802: F, t9760: F) -> (F, F) {
    let t34649 = -0.16666666666666666667e0 * t34346 + 0.26979166666666666667e-1 * t34348 - 0.9375e-1 * t34350 - 0.625e-1 * t34352 + 0.20234375e-1 * t34354 - 0.26979166666666666667e-1 * t34356 + 0.9375e-1 * t34358 - 0.25e0 * t34360 + 0.1875e0 * t34362 - 0.20833333333333333333e-1 * t34364 - 0.625e-1 * t34366 - 0.4046875e-1 * t34369 - 0.26979166666666666667e-1 * t34371;
    let t34650 = t34635 + t34649;
    let t34653 = 2.0 * t10028 * t12345 - t10039 * t5527 - 6.0 * t12352 * t34386 - t2042 * t34650 + t34608 * t802 + 2.0 * t34612 * t5532 + 2.0 * t34615 * t5532 + 2.0 * t34618 * t5532 - t7690 * t9760 + t34289 - t34292 + t34297 + t34302 - t34305 + t34375;
    (t34650, t34653)
}
