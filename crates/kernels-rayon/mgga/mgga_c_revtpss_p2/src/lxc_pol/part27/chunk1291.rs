//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1291/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1291(t25178: f64, t7235: f64, t10416: f64, t7003: f64, t1937: f64, t49693: f64, t13435: f64, t6993: f64, t49856: f64, t18163: f64, t25188: f64, t7239: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95058 = 6.0_f64 * t7235 * t25178;
    let t95066 = 6.0_f64 * t10416 * t7003;
    let t95068 = 6.0_f64 * t49693 * t1937;
    let t95070 = 12.0_f64 * t13435 * t6993;
    let t95073 = 2.0_f64 * t49856 * t1937;
    let t95075 = 6.0_f64 * t18163 * t6993;
    let t95081 = 9.0_f64 * t25188 * t7239;
    (t95058, t95066, t95068, t95070, t95073, t95075, t95081)
}
