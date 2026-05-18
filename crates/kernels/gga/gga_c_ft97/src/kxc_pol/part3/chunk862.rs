//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 862/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk862<F: Float>(t1060: F, t2185: F, t3450: F, t3455: F, t3578: F, t574: F, t12664: F, t3483: F, t144: F, t3478: F, t4790: F, t604: F) -> (F, F, F, F, F, F) {
    let t17394 = t2185 * t1060 * t3450;
    let t17398 = t574 * t3578 * t3455;
    let t17401 = t12664 * t3483;
    let t17402 = t144 * t17401;
    let t17406 = t574 * t3578 * t3478;
    let t17409 = t4790 * t604;
    (t17394, t17398, t17401, t17402, t17406, t17409)
}
