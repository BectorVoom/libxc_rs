//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2178/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2178<F: Float>(t40018: F, t6353: F, t12189: F, t6358: F, t16081: F, t19795: F, t1307: F, t54718: F, t56463: F, t686: F, t16094: F, t16095: F, t5187: F) -> (F, F, F, F, F) {
    let t56484 = t40018 * t6353;
    let t56491 = t12189 * t6358;
    let t56493 = t16081 * t19795;
    let t56501 = t54718 * t686 * t56463 * t1307;
    let t56505 = t16094 * t686 * t16095 * t5187;
    (t56484, t56491, t56493, t56501, t56505)
}
