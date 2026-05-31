//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 367/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk367<F: Float>(t1636: F, t1835: F, t1379: F, t435: F, t690: F, t579: F, t79: F) -> (F, F, F) {
    let t1836 = t1835 * t1636;
    let t1841 = F::cast_from(0.7925e-3_f64) * t435 * t1379 * t690;
    let t1842 = t79 * t579;
    (t1836, t1841, t1842)
}
