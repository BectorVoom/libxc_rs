//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1349/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1349<F: Float>(t20160: F, t33433: F, t9446: F, t1327: F, t32069: F, t6204: F, t6211: F, t20617: F, t9427: F, t33601: F, t3739: F, t109420: F, t1411: F, t33596: F, t32033: F, t3936: F) -> (F, F, F, F, F, F) {
    let t113650 = 0.13888888888888888889e-1 * t9446 * t20160 * t33433;
    let t113657 = t6204 * t32069 * t6211 * t1327;
    let t113663 = t6204 * t9427 * t20617;
    let t113666 = t3739 * t33601;
    let t113669 = t1411 * t109420 * t33596;
    let t113671 = t3936 * t32033;
    (t113650, t113657, t113663, t113666, t113669, t113671)
}
