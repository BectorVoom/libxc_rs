//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1335/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1335<F: Float>(t1261: F, t21192: F, t1238: F, t12882: F, t12893: F, t12900: F, t12905: F, t12985: F, t17509: F, t17546: F, t17556: F, t21177: F, t21184: F, t21189: F, t3711: F) -> F {
    let t21193 = t1261 * t21192;
    let t21196 = -F::new(0.72409452821628889107e-2) * t21177 * t1238 + F::new(0.31758531939310916275e-4) * t12882 - F::new(0.47637797908966374413e-4) * t12893 + t12900 + F::new(0.14291339372689912324e-3) * t3711 * t21184 - F::new(0.47637797908966374413e-4) * t12905 + F::new(0.28582678745379824648e-3) * t21189 - t17509 - F::new(0.19055119163586549765e-3) * t21193 + t17546 + t17556 + F::new(0.47637797908966374413e-4) * t12985;
    t21196
}
