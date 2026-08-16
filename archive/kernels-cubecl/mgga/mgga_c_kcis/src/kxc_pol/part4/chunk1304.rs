//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1304/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1304<F: Float>(t1495: F, t16721: F, t4123: F, t1464: F, t3797: F, t5632: F, t1395: F, t4153: F, t4142: F, t5776: F, t11913: F, t5650: F) -> (F, F, F, F, F) {
    let t16722 = t1495 * t16721;
    let t16723 = t4123 * t16722;
    let t16724 = t1464 * t16723;
    let t16726 = t5632 * t3797;
    let t16727 = t1395 * t16726;
    let t16728 = t4153 * t16727;
    let t16730 = t4142 * t5776;
    let t16731 = F::cast_from(0.22109259259259259258e-2_f64) * t16730;
    let t16732 = t11913 * t5650;
    (t16724, t16728, t16730, t16731, t16732)
}
