//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 760/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk760<F: Float>(t1577: F, t6231: F, t1593: F, t1632: F, t551: F, t1592: F, t2090: F, t57: F, t128: F, t524: F, t540: F, t108: F, t489: F) -> (F, F, F, F, F, F, F) {
    let t6232 = t1577 * t6231;
    let t6235 = t551 * t1632 * t1593;
    let t6236 = t1592 * t6235;
    let t6238 = t2090 * t57;
    let t6239 = t6238 * t128;
    let t6240 = t524 * t6239;
    let t6241 = t6240 * t540;
    let t6243 = t489 * t108;
    (t6232, t6236, t6238, t6239, t6240, t6241, t6243)
}
