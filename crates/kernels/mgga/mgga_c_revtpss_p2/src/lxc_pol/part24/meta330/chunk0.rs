//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1150/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1150<F: Float>(t45: F, t190: F, t22688: F, t10439: F, t4546: F, t5966: F, t18540: F, t18545: F, t18547: F, t14363: F, t22671: F, t4328: F, t5825: F, t633: F, t766: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t23121 = t190 * t22688;
    let t23123 = F::new(24.0) * t10439 * t23121;
    let t23124 = t4546 * t5966;
    let t23127 = F::new(36.0) * t18540;
    let t23128 = F::new(12.0) * t18545;
    let t23129 = F::new(24.0) * t18547;
    let t23130 = F::cast_from(0.32530743900905219526e-1_f64) * t14363;
    let t23138 = piecewise3::<F>(t151, F::new(0.0), F::new(8.0) / F::new(27.0) * t633 * t22688 - F::new(2.0) / F::new(3.0) * t4328 * t5825 + F::new(2.0) / F::new(3.0) * t766 * t22671);
    (t23121, t23123, t23124, t23127, t23128, t23129, t23130, t23138)
}
