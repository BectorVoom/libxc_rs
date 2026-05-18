//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 630/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk630<F: Float>(t1725: F, t8729: F, t4911: F, t8697: F, t4915: F, t7076: F, t8684: F, t8687: F, t8690: F, t2430: F, t1746: F, t4928: F) -> (F, F, F, F, F) {
    let t8730 = t8729 * t1725;
    let t8733 = t8697 * t4911;
    let t8740 = t4915 + F::new(0.61805555555555555556e-2) * t7076 - F::new(0.61805555555555555555e-2) * t8684 + F::new(0.18541666666666666667e-1) * t8687 - F::new(0.92708333333333333333e-2) * t8690;
    let t8746 = t2430 * t2430;
    let t8748 = t4928 * t8746 * t1746;
    (t8730, t8733, t8740, t8746, t8748)
}
