//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1178/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1178<F: Float>(t27126: F, t8461: F, t1583: F, t7086: F, t27383: F, t1544: F, t25207: F, t605: F, t7782: F, t890: F, t1468: F, t775: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t125950 = F::cast_from(2.0_f64) * t27126 * t8461;
    let t125961 = t1583 * t7086;
    let t125962 = t27383 * t125961;
    let t125984 = t1544 * t7086;
    let t125985 = t25207 * t125984;
    let t126007 = t605 * t7782;
    let t126017 = t7782 * t890;
    let t126018 = t27383 * t126017;
    let t126027 = t1468 * t7086;
    let t126030 = t7782 * t775;
    (t125950, t125961, t125962, t125984, t125985, t126007, t126017, t126018, t126027, t126030)
}
