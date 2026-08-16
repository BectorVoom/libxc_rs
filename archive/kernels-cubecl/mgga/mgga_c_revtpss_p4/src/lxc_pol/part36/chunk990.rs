//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 990/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk990<F: Float>(t150: F, t23210: F, t190: F, t1469: F, t18305: F, t4401: F, t14613: F, t6002: F, t22671: F, t706: F, t10592: F, t10596: F, t10604: F, t10611: F, t23193: F, t23213: F, t9542: F) -> (F, F, F, F, F) {
    let t23214 = t150 * t23210;
    let t23215 = t23214 * t190;
    let t23216 = t18305 * t1469;
    let t23218 = F::cast_from(36.0_f64) * t4401 * t23216;
    let t23220 = F::cast_from(36.0_f64) * t14613 * t6002;
    let t23221 = t190 * t22671;
    let t23223 = F::cast_from(4.0_f64) * t706 * t23221;
    let t23224 = t10592 + t23193 - t10596 - t10604 + t23213 + t23215 + t9542 + t23218 + t23220 - t10611 + t23223;
    (t23215, t23218, t23220, t23223, t23224)
}
