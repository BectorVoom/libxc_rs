//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3110/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3110(t12784: f64, t17384: f64, t12772: f64, t17668: f64, t3625: f64, t17673: f64, t12910: f64, t12916: f64, t17460: f64, t17213: f64, t3172: f64, t5384: f64) -> (f64, f64, f64, f64, f64) {
    let t57164 = t12784 * t17384;
    let t57167 = t3625 * t12772 * t17668;
    let t57170 = t3625 * t12772 * t17673;
    let t57173 = t12910 * t12916 * t17460;
    let t57176 = t5384 * t3172 * t17213;
    (t57164, t57167, t57170, t57173, t57176)
}
