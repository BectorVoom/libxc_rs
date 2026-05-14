//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 754/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk754<F: Float>(t3455: F, t3578: F, t574: F, t12664: F, t3483: F, t144: F, t3478: F, t4790: F, t604: F, t609: F, t2185: F, t4668: F, t616: F, t4724: F, t9276: F, t1882: F, t4811: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17398 = t574 * t3578 * t3455;
    let t17401 = t12664 * t3483;
    let t17402 = t144 * t17401;
    let t17406 = t574 * t3578 * t3478;
    let t17409 = t4790 * t604;
    let t17410 = t17409 * t609;
    let t17411 = t144 * t17410;
    let t17415 = t2185 * t616 * t4668;
    let t17418 = t9276 * t4724;
    let t17419 = t144 * t17418;
    let t17422 = t1882 * t4811;
    (t17398, t17401, t17402, t17406, t17410, t17411, t17415, t17418, t17419, t17422)
}
