//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1104/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1104(t30555: f64, t30625: f64, t3: f64, t2055: f64, t5883: f64, t1518: f64, t28986: f64, t5920: f64, t7553: f64, t117: f64, t30570: f64, t1916: f64, t1918: f64, t2113: f64, t2115: f64, t572: f64, t573: f64, t6941: f64, t6945: f64, t6948: f64, t8118: f64, t8124: f64, t8127: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30626 = t30555 + t30625;
    let t30627 = t3 * t30626;
    let t30637 = param_d * t30626;
    let t30651 = t5883 * t2055;
    let t30654 = t28986 * t1518;
    let t30657 = t7553 * t5920;
    let t30660 = t117 * t30570;
    let t30663 = 12.0_f64 * t1916 * t8124 + 6.0_f64 * t1916 * t8127 + 6.0_f64 * t1918 * t8118 + 6.0_f64 * t2113 * t6945 + 3.0_f64 * t2113 * t6948 + 3.0_f64 * t2115 * t6941 + t30637 * t573 + 6.0_f64 * t30651 * t572 + 12.0_f64 * t30654 * t572 + 6.0_f64 * t30657 * t572 + 3.0_f64 * t30660 * t572;
    (t30626, t30627, t30637, t30651, t30654, t30657, t30660, t30663)
}
