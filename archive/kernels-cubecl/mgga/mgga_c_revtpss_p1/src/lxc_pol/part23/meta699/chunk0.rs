//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2448/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2448<F: Float>(t1331: F, t9855: F, t3825: F, t9586: F, t1333: F, t9342: F, t521: F, t583: F, t596: F, t525: F, t9603: F, t527: F, t9615: F) -> (F, F, F, F, F, F) {
    let t47007 = t9855 * t1331;
    let t47011 = t3825 * t9586;
    let t47013 = t9342 * t1333;
    let t47019 = t583 * t596 * t521;
    let t47025 = F::cast_from(1.0_f64) / t525 / t9603;
    let t47040 = F::cast_from(1.0_f64) / t527 / t9615;
    (t47007, t47011, t47013, t47019, t47025, t47040)
}
