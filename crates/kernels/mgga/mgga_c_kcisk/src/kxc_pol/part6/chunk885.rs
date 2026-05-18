//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 885/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk885<F: Float>(t1869: F, t28775: F, t15936: F, t8780: F, t1800: F, t2509: F, t8858: F, t415: F, t11197: F, t23947: F, t23949: F, t23951: F, t28262: F, t28532: F, t28758: F, t28762: F, t28765: F, t28768: F, t671: F) -> (F, F, F, F) {
    let t28776 = t1869 * t28775;
    let t28778 = t15936 * t8780;
    let t28779 = t1800 * t28778;
    let t28780 = t1869 * t28779;
    let t28782 = t2509 * t8858;
    let t28783 = t415 * t28782;
    let t28785 = t28532 * t671 + F::new(0.16581944444444444444e-2) * t28758 + F::new(0.73697530864197530861e-2) * t28762 + F::new(0.49745833333333333332e-2) * t28765 + F::new(0.49745833333333333332e-2) * t28768 - F::new(0.43134342e-1) * t11197 * t28262 + F::new(0.66327777777777777776e-2) * t23947 - F::new(0.17687407407407407407e-1) * t23949 - F::new(0.66327777777777777775e-2) * t23951 + F::new(0.39796666666666666665e-1) * t28776 + F::new(0.44218518518518518518e-2) * t28780 + F::new(0.72960555555555555553e-1) * t28783;
    (t28776, t28780, t28783, t28785)
}
