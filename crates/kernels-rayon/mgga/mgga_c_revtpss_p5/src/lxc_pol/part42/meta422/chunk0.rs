//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1484/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1484(t3: f64, t31700: f64, t2198: f64, t5883: f64, t1518: f64, t31505: f64, t5920: f64, t8342: f64, t117: f64, t31653: f64, t1916: f64, t1918: f64, t2207: f64, t2209: f64, t572: f64, t573: f64, t6941: f64, t6945: f64, t6948: f64, t8421: f64, t8427: f64, t8430: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31701 = t3 * t31700;
    let t31711 = param_d * t31700;
    let t31725 = t5883 * t2198;
    let t31728 = t31505 * t1518;
    let t31731 = t8342 * t5920;
    let t31734 = t117 * t31653;
    let t31737 = 12.0_f64 * t1916 * t8427 + 6.0_f64 * t1916 * t8430 + 6.0_f64 * t1918 * t8421 + 6.0_f64 * t2207 * t6945 + 3.0_f64 * t2207 * t6948 + 3.0_f64 * t2209 * t6941 + t31711 * t573 + 6.0_f64 * t31725 * t572 + 12.0_f64 * t31728 * t572 + 6.0_f64 * t31731 * t572 + 3.0_f64 * t31734 * t572;
    (t31701, t31711, t31725, t31728, t31731, t31734, t31737)
}
