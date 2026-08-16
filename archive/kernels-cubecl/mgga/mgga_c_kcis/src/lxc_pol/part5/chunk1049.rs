//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1049/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1049<F: Float>(t1017: F, t16690: F, t86: F, t11418: F, t556: F, t3728: F, t5673: F, t4142: F, t5776: F, t11913: F, t5650: F, t1363: F, t5623: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16692 = t86 * t1017 * t16690;
    let t16693 = t556 * t11418;
    let t16719 = t3728 * t5673;
    let t16720 = F::cast_from(0.22109259259259259258e-2_f64) * t16719;
    let t16730 = t4142 * t5776;
    let t16731 = F::cast_from(0.22109259259259259258e-2_f64) * t16730;
    let t16732 = t11913 * t5650;
    let t16733 = F::cast_from(0.14739506172839506172e-2_f64) * t16732;
    let t16744 = t5623 * t1363;
    (t16692, t16693, t16719, t16720, t16730, t16731, t16732, t16733, t16744)
}
