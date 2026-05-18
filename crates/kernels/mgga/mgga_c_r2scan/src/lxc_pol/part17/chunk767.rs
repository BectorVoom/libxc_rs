//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 767/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk767<F: Float>(t2271: F, t2813: F, t2452: F, t410: F, t2484: F, t406: F, t1416: F, t899: F, t1419: F, t2483: F, t457: F, t41: F) -> (F, F, F, F, F, F, F) {
    let t7050 = F::new(0.4726e1) * t2271 * t2813;
    let t7051 = t410 * t2452;
    let t7094 = t406 * t2484;
    let t7095 = F::new(8.0) * t7094;
    let t7096 = t410 * t2484;
    let t7097 = F::new(8.0) * t7096;
    let t7109 = t1416 * t899;
    let t7111 = t1419 * t899;
    let t7124 = t2483 * t457;
    let t7125 = t41 * t7124;
    (t7050, t7051, t7095, t7097, t7109, t7111, t7125)
}
