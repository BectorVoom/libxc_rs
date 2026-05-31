//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 892/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk892<F: Float>(t3868: F, t4150: F, t118: F, t1310: F, t1315: F, t1453: F, t2320: F, t2322: F, t2328: F, t2331: F, t2372: F, t3813: F, t3821: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F) -> (F, F) {
    let t4151 = t3868 + t4150;
    let t4153 = -t118 * t3813 - F::cast_from(2.0_f64) * t1310 * t649 + F::cast_from(2.0_f64) * t1315 * t1453 - t2320 * t508 - F::cast_from(4.0_f64) * t2322 * t671 - F::cast_from(2.0_f64) * t2328 * t508 - F::cast_from(4.0_f64) * t2331 * t651 - F::cast_from(2.0_f64) * t2372 * t651 + t3821 * t569 + t4151 * t511;
    (t4151, t4153)
}
