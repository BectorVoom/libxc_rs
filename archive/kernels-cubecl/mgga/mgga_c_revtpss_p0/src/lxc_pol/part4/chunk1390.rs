//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1390/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1390<F: Float>(t12916: F, t5353: F, t3718: F, t5347: F, t3568: F, t471: F, t5351: F, t3720: F, t1781: F, t697: F, t1222: F, t5284: F, t73: F) -> (F, F, F, F, F) {
    let t17617 = t12916 * t5353;
    let t17619 = F::cast_from(0.28582678745379824648e-3_f64) * t3718 * t17617;
    let t17620 = t12916 * t5347;
    let t17622 = F::cast_from(0.28582678745379824648e-3_f64) * t3718 * t17620;
    let t17623 = t471 * t3568;
    let t17624 = t5351 * t17623;
    let t17625 = t3720 * t17624;
    let t17628 = t697 * t1781;
    let t17629 = t1222 * t17628;
    let t17633 = t5284 * t73;
    (t17619, t17622, t17625, t17629, t17633)
}
