//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 428/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk428(t2010: f64, t2127: f64, t118: f64, t1939: f64, t2036: f64, t2163: f64, t508: f64, t569: f64, t3: f64, t2044: f64, t573: f64, t10: f64, t17: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2165 = t2127 + t2010;
    let t2167 = -t118 * t2163 - t2127 * t508 + t2165 * t569 - t1939 + t2036;
    let t2168 = t3 * t2167;
    let t2170 = param_d * t2167;
    let t2172 = t2170 * t573 + t2044;
    let t2219 = 2.0_f64 * t10 * t17;
    (t2165, t2167, t2168, t2170, t2172, t2219)
}
