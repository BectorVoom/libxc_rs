//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1197/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1197(t7334: f64, t7944: f64, t7324: f64, t7953: f64, t7950: f64, t1459: f64, t34007: f64, t1916: f64, t32366: f64, t127412: f64, t127480: f64, t127481: f64, t127483: f64, t127489: f64, t127490: f64, t127492: f64, t127495: f64, t32373: f64, t34011: f64, t34014: f64, t573: f64, t5805: f64, t8607: f64, t8616: f64, param_d: f64) -> f64 {
    let t127496 = t7944 * t7334;
    let t127498 = t7324 * t7953;
    let t127500 = t7324 * t7950;
    let t127503 = 12.0_f64 * t1459 * t34007;
    let t127507 = 6.0_f64 * t1916 * t32366;
    let t127508 = t127412 * t573 * param_d + 3.0_f64 * t5805 * t8607 + t127480 + 12.0_f64 * t127481 + 12.0_f64 * t127483 + t127489 + 12.0_f64 * t127490 + 6.0_f64 * t127492 + t127495 + 6.0_f64 * t127496 + 6.0_f64 * t127498 + 12.0_f64 * t127500 + t127503 + t127507 + t32373 + t34011 + t34014 + t8616;
    t127508
}
