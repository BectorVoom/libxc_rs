//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1237/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1237<F: Float>(t32980: F, t8673: F, t415: F, t8677: F, t9687: F, t717: F, t8874: F, t20: F, t648: F, t8831: F, t1693: F, t2528: F, t34173: F, t2509: F, t2537: F, t2785: F, t33031: F, t34073: F, t34083: F, t34098: F, t34113: F, t34119: F, t34122: F, t34140: F, t34154: F, t35123: F, t35136: F, t9664: F, t9922: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t35180 = t32980 * t8673;
    let t35181 = t415 * t35180;
    let t35183 = t9687 * t8677;
    let t35184 = t415 * t35183;
    let t35186 = t717 * t8874;
    let t35187 = t415 * t35186;
    let t35191 = t648 * t8831 * t20;
    let t35192 = t1693 * t35191;
    let t35202 = t34173 * t2528;
    let t35203 = t415 * t35202;
    let t35205 = t2509 * t2537;
    let t35206 = t415 * t35205;
    let t35210 = 0.18518518518518518519e-1 * t34083 + 0.20833333333333333334e-1 * t34073 * t9922 + 0.8041666666666666667e-2 * t34154 * t9922 + 0.49745833333333333332e-2 * t35181 + 0.13265555555555555555e-1 * t35184 - 0.55273148148148148147e-3 * t35187 + 0.69444444444444444446e-2 * t34098 - 0.10185185185185185186e0 * t35192 * t2785 - 0.69444444444444444446e-2 * t34113 - 0.23148148148148148148e-2 * t34119 + 0.20833333333333333334e-1 * t34122 * t9922 + 0.22109259259259259258e-2 * t34140 + 0.69444444444444444446e-2 * t33031 * t35123 - 0.49745833333333333332e-2 * t35203 + 0.33163888888888888888e-2 * t35206 + 0.10416666666666666667e-1 * t9664 * t35136;
    (t35180, t35181, t35183, t35184, t35186, t35187, t35191, t35192, t35202, t35203, t35205, t35206, t35210)
}
