//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 363/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk363<F: Float>(t529: F, t1216: F, t41: F, t1287: F, t382: F, t525: F, t526: F, t79: F, t534: F) -> (F, F, F, F) {
    let t530 = t529 < -0.66725e-1;
    let t1558 = t1216 * t41;
    let t1566 = piecewise3(t530, 0.0, 10.0 / 9.0 * t525 * t1558 * t382 - 10.0 / 27.0 * t525 * t526 * t1287);
    let t1567 = t79 * t1566;
    let t1568 = t1567 * t534;
    (t1558, t1566, t1567, t1568)
}
