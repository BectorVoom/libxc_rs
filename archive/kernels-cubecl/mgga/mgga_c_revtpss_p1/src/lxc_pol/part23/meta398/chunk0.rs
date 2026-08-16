//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1758/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1758<F: Float>(t17376: F, t3599: F, t3704: F, t5274: F, t1285: F, t17395: F) -> (F, F, F) {
    let t17572 = t17376 * t3599;
    let t17593 = F::cast_from(0.28582678745379824648e-3_f64) * t5274 * t3704;
    let t17605 = t1285 * t17395;
    (t17572, t17593, t17605)
}
