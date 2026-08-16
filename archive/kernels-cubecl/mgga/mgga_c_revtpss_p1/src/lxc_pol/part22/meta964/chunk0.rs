//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3226/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3226<F: Float>(t14383: F, t4311: F, t40092: F, t40094: F, t50047: F, t14386: F, t4305: F, t1544: F, t2832: F, t157: F, t2251: F, t6002: F) -> (F, F, F, F, F, F, F) {
    let t61197 = F::cast_from(8.0_f64) * t4311 * t14383;
    let t61198 = F::cast_from(0.10389515463408878255e3_f64) * t40092;
    let t61199 = F::cast_from(0.70178683471615754484e1_f64) * t40094;
    let t61200 = F::cast_from(0.97661052298701573622e-3_f64) * t50047;
    let t61201 = t14386 * t4305;
    let t61202 = F::cast_from(16.0_f64) * t61201;
    let t61203 = t1544 * t2832;
    let t61209 = F::cast_from(24.0_f64) * t2251 * t157 * t6002;
    (t61197, t61198, t61199, t61200, t61202, t61203, t61209)
}
