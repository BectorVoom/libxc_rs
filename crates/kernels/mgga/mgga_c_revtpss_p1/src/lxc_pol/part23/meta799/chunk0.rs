//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2624/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2624<F: Float>(t40398: F, t6024: F, t18435: F, t221: F, t10703: F, t2674: F, t14832: F, t2661: F, t62351: F, t775: F, t10716: F, t18423: F) -> (F, F, F, F) {
    let t62401 = t40398 * t6024;
    let t62403 = t221 * t18435;
    let t62405 = t2674 * t10703 * t62403;
    let t62429 = t2661 * t14832 * t62351 * t775;
    let t62431 = t10716 * t18423;
    (t62401, t62405, t62429, t62431)
}
