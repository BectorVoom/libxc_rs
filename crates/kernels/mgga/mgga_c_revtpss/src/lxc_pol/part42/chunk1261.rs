//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1261/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1261<F: Float>(t1234: F, t6594: F, t1214: F, t5825: F, t5296: F, t1042: F, t3172: F, t6630: F, t3600: F, t247: F, t3634: F, t6425: F, t1261: F, t1238: F, t12882: F, t12893: F, t12900: F, t12905: F, t12985: F, t17509: F, t17546: F, t17556: F, t3711: F) -> (F,) {
    let t21177 = t1234 * t6594;
    let t21182 = t5825 * t1214;
    let t21183 = t5296 * t21182;
    let t21184 = t1042 * t21183;
    let t21188 = t3172 * t6630;
    let t21189 = t3600 * t21188;
    let t21192 = t247 * t3634 * t6425;
    let t21193 = t1261 * t21192;
    let t21196 = -0.72409452821628889107e-2 * t21177 * t1238 + 0.31758531939310916275e-4 * t12882 - 0.47637797908966374413e-4 * t12893 + t12900 + 0.14291339372689912324e-3 * t3711 * t21184 - 0.47637797908966374413e-4 * t12905 + 0.28582678745379824648e-3 * t21189 - t17509 - 0.19055119163586549765e-3 * t21193 + t17546 + t17556 + 0.47637797908966374413e-4 * t12985;
    (t21196,)
}
