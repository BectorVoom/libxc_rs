//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1075/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1075<F: Float>(t132: F, t3979: F, t7198: F, t2456: F, t3988: F, t10621: F, t1875: F, t3480: F, t674: F, t926: F, t222: F, t37: F, zeta_threshold: F) -> (F, F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t11269 = t7198 * t3979;
    let t11274 = t2456 * t3988;
    let t11280 = piecewise3(t133, 0.0, -28.0 / 27.0 * t11269 * t674 - 16.0 / 9.0 * t3480 * t1875 + 4.0 / 9.0 * t11274 * t674 - t926 * t10621 / 3.0);
    let t11282 = t222 * t37 * t11280;
    (t11269, t11274, t11280, t11282)
}
