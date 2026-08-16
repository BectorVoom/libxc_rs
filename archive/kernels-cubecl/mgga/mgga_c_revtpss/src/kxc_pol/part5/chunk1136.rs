//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1136/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1136<F: Float>(t1260: F, t5261: F, t3647: F, t5378: F, t247: F, t3634: F, t5056: F, t1261: F, t12916: F, t5334: F, t5331: F, t1778: F, t3682: F) -> (F, F, F, F, F) {
    let t17763 = t5261 * t1260;
    let t17767 = F::cast_from(0.19055119163586549765e-3_f64) * t3647 * t5378;
    let t17769 = t247 * t3634 * t5056;
    let t17771 = F::cast_from(0.19055119163586549765e-3_f64) * t1261 * t17769;
    let t17789 = t12916 * t5334;
    let t17791 = F::cast_from(0.28582678745379824648e-3_f64) * t5331 * t17789;
    let t17792 = t1778 * t3682;
    (t17763, t17767, t17771, t17791, t17792)
}
