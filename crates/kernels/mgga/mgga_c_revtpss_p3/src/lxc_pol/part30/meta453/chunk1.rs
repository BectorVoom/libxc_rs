//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1724/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1724<F: Float>(t3647: F, t5378: F, t247: F, t3634: F, t5056: F, t1261: F, t1266: F, t17721: F, t17724: F, t17729: F, t17732: F, t17736: F, t17739: F, t17744: F, t17747: F, t17750: F, t17753: F, t17756: F, t17760: F, t17763: F, t3718: F) -> (F, F) {
    let t17767 = F::cast_from(0.19055119163586549765e-3_f64) * t3647 * t5378;
    let t17769 = t247 * t3634 * t5056;
    let t17771 = F::cast_from(0.19055119163586549765e-3_f64) * t1261 * t17769;
    let t17772 = F::cast_from(0.31758531939310916276e-3_f64) * t17721 - F::cast_from(0.42874018118069736972e-3_f64) * t3718 * t17724 + F::cast_from(0.57165357490759649296e-3_f64) * t17729 * t17732 - F::cast_from(0.57165357490759649296e-3_f64) * t17736 * t17739 - F::cast_from(0.21437009059034868486e-3_f64) * t3718 * t17744 - F::cast_from(0.12862205435420921092e-2_f64) * t17747 * t17750 + F::cast_from(0.21437009059034868486e-3_f64) * t17753 * t17756 - F::cast_from(0.47637797908966374414e-3_f64) * t17729 * t17760 - F::cast_from(0.28582678745379824648e-3_f64) * t17763 * t1266 - t17767 - t17771;
    (t17769, t17772)
}
