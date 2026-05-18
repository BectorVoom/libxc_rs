//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1010/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1010<F: Float>(t33: F, t265: F, t502: F, t32058: F, t32088: F, t57: F, t606: F, t8553: F, t531: F, t8594: F, t7238: F, t2014: F, t7235: F, t8600: F, t2007: F, t7002: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t32089 = piecewise3::<f64>(t503, F::new(0.0), t32058);
    let t32094 = piecewise3::<f64>(t400, t32088, t32089 * t57 / F::new(2.0) - t8553 * t606 / F::new(2.0));
    let t32098 = t531 * t8594;
    let t32099 = t32098 * t7238;
    let t32101 = F::new(3.0) * t2014 * t32099;
    let t32102 = t7235 * t8600;
    let t32103 = t2007 * t7002;
    (t32089, t32094, t32098, t32099, t32101, t32102, t32103)
}
