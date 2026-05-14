//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1170/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1170<F: Float>(t33155: F, t7584: F, t7585: F, t10848: F, t22748: F, t32356: F, t701: F, t20157: F, t323: F, t32349: F, t320: F, t32608: F, t831: F, t6066: F, t7630: F, t10860: F, t23279: F) -> (F, F, F, F, F, F, F, F) {
    let t33624 = 0.11502877786176224903e2 * t7584 * t7585 * t33155;
    let t33626 = 0.23005755572352449806e2 * t22748 * t10848;
    let t33627 = t32356 * t701;
    let t33630 = 0.23005755572352449806e2 * t7584 * t7585 * t33627;
    let t33633 = 0.40899121017515466321e1 * t323 * t20157 * t32349;
    let t33637 = 0.19427082483319846503e2 * t320 * t831 * t20157 * t32608;
    let t33640 = 0.71500979903700853338e0 * t7630 * t6066 * t33155;
    let t33642 = 0.14300195980740170668e1 * t23279 * t10860;
    (t33624, t33626, t33627, t33630, t33633, t33637, t33640, t33642)
}
