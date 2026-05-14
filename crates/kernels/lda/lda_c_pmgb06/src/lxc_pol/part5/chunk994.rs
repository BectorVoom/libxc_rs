//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 994/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk994<F: Float>(t20613: F, t1447: F, t7674: F, t2485: F, t5187: F, t2002: F, t6250: F, t1420: F, t7574: F, t20601: F, t20602: F, t20603: F, t20604: F, t20608: F, t20610: F, t20612: F) -> (F, F, F, F, F, F) {
    let t20614 = 2.0 / 45.0 * t20613;
    let t20615 = t1447 * t7674;
    let t20616 = 2.0 / 45.0 * t20615;
    let t20618 = t5187 * t2485 / 9.0;
    let t20620 = t2002 * t6250 / 9.0;
    let t20622 = t1420 * t7574 / 15.0;
    let t20623 = -t20601 - t20602 - t20603 + t20604 - t20608 + t20610 + t20612 + t20614 + t20616 + t20618 + t20620 + t20622;
    (t20614, t20616, t20618, t20620, t20622, t20623)
}
