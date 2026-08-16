//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1014/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1014(t32171: f64, t7002: f64, t8692: f64, t2322: f64, t8460: f64, t5523: f64, t7235: f64, t8596: f64, t27: f64, t8571: f64, t221: f64, t4019: f64, t561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32172 = 2.0_f64 * t32171;
    let t32174 = 4.0_f64 * t8692 * t7002;
    let t32175 = t2322 * t8460;
    let t32176 = 2.0_f64 * t32175;
    let t32177 = t5523 * t8460;
    let t32178 = 2.0_f64 * t32177;
    let t32182 = t7235 * t8596;
    let t32183 = t8571 * t27;
    let t32186 = t4019 * t221 * t561;
    (t32172, t32174, t32175, t32176, t32178, t32182, t32183, t32186)
}
