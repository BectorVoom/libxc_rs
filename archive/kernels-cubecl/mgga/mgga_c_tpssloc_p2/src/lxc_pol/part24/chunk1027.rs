//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1027/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1027<F: Float>(t11702: F, t1213: F, t3490: F, t3523: F, t1190: F, t3030: F, t3032: F, t3505: F, t10469: F, t466: F, t10471: F, t1208: F) -> (F, F, F, F, F, F, F, F) {
    let t11703 = t1213 * t11702;
    let t11705 = t3490 * t3523;
    let t11707 = t1190 * t3030;
    let t11708 = t11707 * t3032;
    let t11709 = t11708 * t3505;
    let t11712 = t466 * t10469;
    let t11713 = t11712 * t10471;
    let t11714 = t1208 * t1208;
    (t11703, t11705, t11707, t11708, t11709, t11712, t11713, t11714)
}
