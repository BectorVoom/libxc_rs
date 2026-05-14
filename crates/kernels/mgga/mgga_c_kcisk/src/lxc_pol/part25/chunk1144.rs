//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1144/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1144<F: Float>(t2804: F, t33276: F, t9733: F, t9736: F, t5463: F, t654: F, t20: F, t2801: F, t9720: F, t9732: F, t11940: F, t79: F, t2803: F, t9739: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33278 = 0.11574074074074074074e-2 * t2804 * t33276;
    let t33279 = t9733 * t9736;
    let t33282 = t5463 * t654;
    let t33283 = t33282 * t20;
    let t33284 = t2801 * t33283;
    let t33287 = t9720 * t9732;
    let t33290 = t11940 * t79;
    let t33291 = t33290 * t2803;
    let t33297 = t9720 * t9739;
    (t33278, t33279, t33282, t33283, t33284, t33287, t33290, t33291, t33297)
}
