//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 373/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk373<F: Float>(t604: F, t20: F, t2454: F, t649: F, t1776: F, t2063: F, t1775: F, t2399: F) -> (F, F, F, F, F) {
    let t659 = 0.0 < t604;
    let t2455 = t2454 * t20;
    let t2456 = t649 * t2455;
    let t2459 = t1776 * t2063;
    let t2460 = t1775 * t2459;
    let t2464 = piecewise3(t659, t2399, -t2399);
    (t2455, t2456, t2459, t2460, t2464)
}
