//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 956/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk956<F: Float>(t16622: F, t3728: F, t5634: F, t5758: F, t5417: F, t4135: F, t4169: F, t5877: F, t11670: F, t540: F, t1017: F, t86: F, t11418: F, t556: F, t5673: F, t4142: F, t5776: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16623 = t16622 * sigma2;
    let t16627 = t3728 * t5634;
    let t16628 = 0.88437037037037037034e-2 * t16627;
    let t16629 = t3728 * t5758;
    let t16631 = t3728 * t5417;
    let t16632 = 0.33163888888888888888e-2 * t16631;
    let t16633 = t4169 * t4135;
    let t16663 = t3728 * t5877;
    let t16690 = t11670 * t540;
    let t16692 = t86 * t1017 * t16690;
    let t16693 = t556 * t11418;
    let t16719 = t3728 * t5673;
    let t16720 = 0.22109259259259259258e-2 * t16719;
    let t16730 = t4142 * t5776;
    (t16623, t16627, t16628, t16629, t16631, t16632, t16633, t16663, t16692, t16693, t16719, t16720, t16730)
}
