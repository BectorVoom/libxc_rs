//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 772/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk772<F: Float>(t1131: F, t6491: F, t1096: F, t1092: F, t1773: F) -> (F, F, F, F) {
    let t6492 = t1131 * t6491;
    let t6493 = t1096 * t6492;
    let t6494 = t1092 * t6493;
    let t6496 = t1773 * t1773;
    (t6492, t6493, t6494, t6496)
}
