//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1661/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1661<F: Float>(t1261: F, t21192: F, t1238: F, t12882: F, t12893: F, t12900: F, t12905: F, t12985: F, t17509: F, t17546: F, t17556: F, t21177: F, t21184: F, t21189: F, t3711: F) -> F {
    let t21193 = t1261 * t21192;
    let t21196 = -F::cast_from(0.72409452821628889107e-2_f64) * t21177 * t1238 + F::cast_from(0.31758531939310916275e-4_f64) * t12882 - F::cast_from(0.47637797908966374413e-4_f64) * t12893 + t12900 + F::cast_from(0.14291339372689912324e-3_f64) * t3711 * t21184 - F::cast_from(0.47637797908966374413e-4_f64) * t12905 + F::cast_from(0.28582678745379824648e-3_f64) * t21189 - t17509 - F::cast_from(0.19055119163586549765e-3_f64) * t21193 + t17546 + t17556 + F::cast_from(0.47637797908966374413e-4_f64) * t12985;
    t21196
}
