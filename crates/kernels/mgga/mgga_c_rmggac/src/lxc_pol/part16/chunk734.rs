//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 734/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk734<F: Float>(t1965: F, t9085: F, t1969: F, t2305: F, t35654: F, t16502: F, t8516: F, t5016: F, t9000: F, t1605: F, t1986: F, t118: F, t128: F, t1494: F, t209: F, t1550: F, t5144: F, t7778: F) -> (F, F, F, F, F, F, F) {
    let t39392 = t9085 * t1965;
    let t39393 = t39392 * t1969;
    let t39405 = t35654 * t2305;
    let t39437 = t8516 * t16502;
    let t39451 = t5016 * t9000;
    let t39490 = t1986 * t1605;
    let t39513 = t1986 * t118 * t128 * t1494 * t209;
    let t39528 = t1550 * t7778 * t5144;
    (t39393, t39405, t39437, t39451, t39490, t39513, t39528)
}
