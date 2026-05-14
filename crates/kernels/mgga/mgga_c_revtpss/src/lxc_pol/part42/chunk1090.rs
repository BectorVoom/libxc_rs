//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1090/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1090<F: Float>(t13142: F, t17708: F, t13127: F, t1260: F, t5261: F, t3647: F, t5378: F, t247: F, t3634: F, t5056: F, t1261: F, t12916: F, t5334: F, t5331: F, t1778: F, t3682: F) -> (F, F, F, F, F, F, F) {
    let t17747 = t13142 * t17708;
    let t17753 = t13127 * t17708;
    let t17763 = t5261 * t1260;
    let t17767 = 0.19055119163586549765e-3 * t3647 * t5378;
    let t17769 = t247 * t3634 * t5056;
    let t17771 = 0.19055119163586549765e-3 * t1261 * t17769;
    let t17789 = t12916 * t5334;
    let t17791 = 0.28582678745379824648e-3 * t5331 * t17789;
    let t17792 = t1778 * t3682;
    (t17747, t17753, t17763, t17767, t17771, t17791, t17792)
}
