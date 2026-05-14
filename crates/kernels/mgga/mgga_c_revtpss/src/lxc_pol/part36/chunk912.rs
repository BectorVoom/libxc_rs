//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 912/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk912<F: Float>(t23216: F, t4401: F, t14613: F, t6002: F, t190: F, t22671: F, t706: F, t10592: F, t10596: F, t10604: F, t10611: F, t23193: F, t23213: F, t23215: F, t9542: F, t225: F, t23185: F, t23187: F, t23192: F) -> (F, F, F, F) {
    let t23218 = 36.0 * t4401 * t23216;
    let t23220 = 36.0 * t14613 * t6002;
    let t23221 = t190 * t22671;
    let t23223 = 4.0 * t706 * t23221;
    let t23224 = t10592 + t23193 - t10596 - t10604 + t23213 + t23215 + t9542 + t23218 + t23220 - t10611 + t23223;
    let t23227 = (t23185 + t23187 + t23192 + t23224) * t225;
    (t23218, t23220, t23223, t23227)
}
