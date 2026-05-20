//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2387/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2387<F: Float>(t2448: F, t9292: F, t10994: F, t2453: F, t11043: F, t11029: F, t9303: F, t39501: F, t781: F, t252: F, t257: F, t268: F, t39644: F, t8779: F) -> (F, F, F, F, F, F) {
    let t41004 = t9292 * t2448;
    let t41011 = t2453 * t10994;
    let t41020 = t2453 * t11043;
    let t41034 = t9303 * t11029;
    let t41037 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t781;
    let t41049 = F::cast_from(0.11638313500518478545e-4_f64) * t39644 * t252 * t257 * t8779 * t268;
    (t41004, t41011, t41020, t41034, t41037, t41049)
}
