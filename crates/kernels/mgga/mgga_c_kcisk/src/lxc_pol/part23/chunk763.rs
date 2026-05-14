//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 763/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk763<F: Float>(t1292: F, t20: F, t394: F, t1220: F, t1299: F, t388: F, t2717: F, t4419: F) -> (F, F, F, F, F) {
    let t9433 = t1292 * t394 * t20;
    let t9434 = t1220 * t9433;
    let t9438 = t388 * t1299 * t20;
    let t9439 = t1220 * t9438;
    let t9442 = t4419 * t2717;
    (t9433, t9434, t9438, t9439, t9442)
}
