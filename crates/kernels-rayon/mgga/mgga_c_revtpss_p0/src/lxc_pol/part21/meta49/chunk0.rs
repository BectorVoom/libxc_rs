//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 362/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk362(t300: f64, t311: f64, t912: f64, t938: f64, t941: f64, t946: f64, t955: f64, t961: f64, t965: f64, t974: f64, t315: f64) -> (f64, f64, f64) {
    let t978 = t300 * (-0.310907e-1_f64 * t941 * t311 + 1.0_f64 * t946 * t955 + t912 - t938 - 0.19751673498613801407e-1_f64 * t961 + 0.5848223622634646207e0_f64 * t965 * t974);
    let t980 = 0.19751673498613801407e-1_f64 * t300 * t961;
    let t981 = t300 * t315;
    (t978, t980, t981)
}
