//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1351/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1351<F: Float>(t3585: F, t820: F, t10401: F, t3575: F, t3610: F, t3624: F, t3521: F, t1190: F, t3030: F, t3032: F, t3505: F, t10469: F, t466: F) -> (F, F, F, F, F, F, F, F) {
    let t11668 = t820 * t3585;
    let t11677 = t3575 * t10401;
    let t11678 = t3610 * t11677;
    let t11692 = t3624 * t11677;
    let t11697 = t820 * t3521;
    let t11707 = t1190 * t3030;
    let t11708 = t11707 * t3032;
    let t11709 = t11708 * t3505;
    let t11712 = t466 * t10469;
    (t11668, t11678, t11692, t11697, t11707, t11708, t11709, t11712)
}
