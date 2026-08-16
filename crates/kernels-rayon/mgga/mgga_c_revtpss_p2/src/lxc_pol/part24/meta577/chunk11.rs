//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1780/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1780(t300: f64, t90745: f64, t90775: f64, t90805: f64, t90852: f64, t24488: f64, t5192: f64, t1196: f64, t20890: f64, t69511: f64, t6535: f64, t6555: f64) -> (f64, f64, f64, f64) {
    let t90855 = t300 * (t90745 + t90775 + t90805 + t90852);
    let t90857 = 0.14035736694323150897e2_f64 * t5192 * t24488;
    let t90860 = 0.61524113149298439947e4_f64 * t1196 * t20890 * t69511;
    let t90863 = 0.21053605041484726346e2_f64 * t1196 * t6555 * t6535;
    (t90855, t90857, t90860, t90863)
}
