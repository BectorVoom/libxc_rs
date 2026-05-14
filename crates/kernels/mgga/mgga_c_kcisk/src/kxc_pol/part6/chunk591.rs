//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 591/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk591<F: Float>(t1725: F, t8729: F, t4911: F, t8697: F, t4915: F, t7076: F, t8684: F, t8687: F, t8690: F, t2430: F, t1746: F, t4928: F, t4936: F, t4943: F, t7122: F, t8702: F, t8709: F, t8715: F, t8717: F, t8721: F, t8724: F, t8727: F) -> (F, F, F, F, F, F) {
    let t8730 = t8729 * t1725;
    let t8733 = t8697 * t4911;
    let t8740 = t4915 + 0.61805555555555555556e-2 * t7076 - 0.61805555555555555555e-2 * t8684 + 0.18541666666666666667e-1 * t8687 - 0.92708333333333333333e-2 * t8690;
    let t8746 = t2430 * t2430;
    let t8748 = t4928 * t8746 * t1746;
    let t8763 = -0.1294625e1 * t8702 + 0.258925e1 * t8709 + t4936 + 0.20128333333333333334e0 * t7076 - 0.20128333333333333333e0 * t8684 + 0.60385e0 * t8687 - 0.301925e0 * t8690 + 0.82524375e-1 * t8715 + 0.16504875e0 * t8717 + t4943 + 0.22076e0 * t7122 - 0.5519e-1 * t8721 + 0.33114e0 * t8724 - 0.16557e0 * t8727;
    (t8730, t8733, t8740, t8746, t8748, t8763)
}
