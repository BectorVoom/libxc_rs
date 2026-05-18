//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1125/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1125<F: Float>(t3678: F, t5327: F, t5323: F, t3667: F, t5362: F, t1789: F, t371: F, t676: F, t1235: F, t1769: F, t3565: F, t225: F) -> (F, F, F, F, F, F) {
    let t17296 = F::new(0.28582678745379824648e-3) * t5327 * t3678;
    let t17298 = F::new(0.15244095330869239812e-2) * t5323 * t3678;
    let t17301 = F::new(0.28582678745379824648e-3) * t3667 * t5362;
    let t17303 = t371 * t676 * t1789;
    let t17304 = t1235 * t17303;
    let t17306 = t1769 * t3565;
    let t17307 = t17306 * t225;
    (t17296, t17298, t17301, t17304, t17306, t17307)
}
