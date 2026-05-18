//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 430/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk430<F: Float>(t2116: F, t625: F, t122: F, t2111: F, t409: F, t57: F, t110: F, t20: F, t524: F, t525: F) -> (F, F, F, F) {
    let t2117 = t2116 * t625;
    let t2119 = F::new(0.14457274399185490173e-3) * t2111 * t122 * t409 * t57 * t2117;
    let t2120 = t110 * t20;
    let t2122 = t524 * t525 * t2120;
    (t2117, t2119, t2120, t2122)
}
