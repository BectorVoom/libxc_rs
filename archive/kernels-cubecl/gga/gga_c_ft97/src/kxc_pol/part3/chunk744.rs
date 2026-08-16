//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 744/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk744<F: Float>(t15639: F, t3020: F, t401: F, t4449: F, t383: F, t4441: F, t35: F, t7906: F, t4467: F, t4466: F, t1594: F, t428: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15640 = t3020 * t15639;
    let t15643 = t4449 * t401;
    let t15647 = t4441 * t383;
    let t15648 = t15647 * t35;
    let t15649 = t7906 * t15648;
    let t15652 = t4467 * t401;
    let t15656 = t4466 * t383;
    let t15657 = t15656 * t35;
    let t15658 = t1594 * t15657;
    let t15664 = t4449 * t428;
    (t15640, t15643, t15647, t15648, t15649, t15652, t15656, t15657, t15658, t15664)
}
