//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1967/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1967(t30188: f64, t572: f64, t5920: f64, t7330: f64, t117: f64, t30004: f64, t1918: f64, t2040: f64, t30171: f64, t30180: f64, t30182: f64, t30184: f64, t30187: f64, t573: f64, t6945: f64, t6948: f64, t7944: f64) -> (f64, f64, f64) {
    let t30190 = 12.0_f64 * t572 * t30188;
    let t30191 = t7330 * t5920;
    let t30193 = 6.0_f64 * t572 * t30191;
    let t30194 = t117 * t30004;
    let t30196 = 3.0_f64 * t572 * t30194;
    let t30197 = 6.0_f64 * t1918 * t7944 + 6.0_f64 * t2040 * t6945 + 3.0_f64 * t2040 * t6948 + t30171 * t573 + t30180 + t30182 + t30184 + t30187 + t30190 + t30193 + t30196;
    (t30191, t30194, t30197)
}
