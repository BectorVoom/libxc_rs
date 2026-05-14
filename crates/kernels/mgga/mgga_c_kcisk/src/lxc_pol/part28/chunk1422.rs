//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1422/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1422<F: Float>(t2803: F, t74221: F, t79: F, t122596: F, t33177: F, t117652: F, t24991: F, t7552: F, t117621: F, t117897: F, t118275: F, t1636: F, t25052: F, t33196: F, t33219: F, t33220: F, t34435: F, t34496: F, t34501: F, t34548: F, t34552: F, t34563: F, t35431: F, t35467: F, t9728: F, t9740: F) -> (F, F) {
    let t122653 = t74221 * t79 * t2803;
    let t122656 = t33177 * t122596;
    let t122681 = t117652 * t7552 * t24991;
    let t122684 = 0.52083333333333333333e-2 * t35431 * t9728 + 0.20104166666666666667e-2 * t122653 * t9728 - 0.38801041666666666667e-3 * t122656 + 0.34722222222222222222e-2 * t34435 * t34552 + 0.34722222222222222222e-2 * t34435 * t34496 + 0.69444444444444444444e-2 * t34435 * t34501 + 0.13402777777777777778e-2 * t117621 * t34496 - 0.46296296296296296296e-2 * t34435 * t34563 + 0.17361111111111111111e-2 * t9740 * t33219 * t33220 * t25052 + 0.13402777777777777778e-2 * t117621 * t34548 + 0.17361111111111111111e-2 * t9740 * t33219 * t35467 * t1636 + 0.13402777777777777778e-2 * t118275 * t34548 + 0.80416666666666666668e-2 * t33196 * t122681 + t117897;
    (t122681, t122684)
}
