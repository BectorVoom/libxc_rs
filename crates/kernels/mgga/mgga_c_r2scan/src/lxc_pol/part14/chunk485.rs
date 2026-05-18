//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 485/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk485<F: Float>(t1234: F, t552: F, t551: F, t110: F, t1598: F, t524: F, t531: F, t108: F, t144: F, t543: F) -> (F, F, F, F, F, F) {
    let t2172 = t552 * t1234;
    let t2173 = t551 * t2172;
    let t2176 = t1598 * t110;
    let t2177 = t524 * t2176;
    let t2178 = t2177 * t531;
    let t2182 = t108 / t543 / t144;
    (t2172, t2173, t2176, t2177, t2178, t2182)
}
