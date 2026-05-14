//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1086/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1086<F: Float>(t1285: F, t17395: F, t1032: F, t5216: F, t1246: F, t12916: F, t5353: F, t3718: F, t5347: F, t1781: F, t697: F, t1222: F, t5284: F, t73: F, t17350: F, t3767: F) -> (F, F, F, F, F, F, F) {
    let t17605 = t1285 * t17395;
    let t17608 = t5216 * t1032;
    let t17609 = t17608 * t1246;
    let t17617 = t12916 * t5353;
    let t17619 = 0.28582678745379824648e-3 * t3718 * t17617;
    let t17620 = t12916 * t5347;
    let t17622 = 0.28582678745379824648e-3 * t3718 * t17620;
    let t17628 = t697 * t1781;
    let t17629 = t1222 * t17628;
    let t17633 = t5284 * t73;
    let t17654 = t3767 * t17350;
    (t17605, t17609, t17619, t17622, t17629, t17633, t17654)
}
