//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 843/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk843<F: Float>(t5384: F, t603: F, t586: F, t158: F, t164: F, t1721: F, t499: F, t52: F, t146: F, t155: F, t95: F, t625: F, t1975: F, t712: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5385 = t5384 * t603;
    let t5387 = t586 * t586;
    let t5388 = 1.0 / t5387;
    let t5389 = t158 * t5388;
    let t5391 = t1721 * t164;
    let t5401 = 1.0 / t52 / t499;
    let t5402 = t146 * t5401;
    let t5405 = 455.0 / 1296.0 * t5402 * t95 * t155;
    let t5417 = t625 * t625;
    let t5418 = 1.0 / t5417;
    let t5490 = 1.0 / t1975 / t712;
    (t5385, t5387, t5389, t5391, t5401, t5402, t5405, t5417, t5418, t5490)
}
