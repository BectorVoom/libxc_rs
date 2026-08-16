//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1212/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1212(t122003: f64, t27186: f64, t34049: f64, t686: f64, t72: f64, t32474: f64, t32469: f64, t119992: f64, t120003: f64, t120014: f64, t120017: f64, t121942: f64, t121946: f64, t121975: f64, t1579: f64, t27350: f64, t31812: f64, t32440: f64, t8649: f64) -> f64 {
    let t127794 = t122003 * t27186;
    let t127798 = t34049 * t72 * t686;
    let t127799 = t32474 * t127798;
    let t127801 = t32469 * t127798;
    let t127807 = -0.17135921299530705785e1_f64 * t8649 * t31812 * t32440 * t1579 + 0.25702851531048074406e-1_f64 * t127794 - 0.14456046980341999104e-1_f64 * t121942 + t121946 + t119992 + 0.25389723392137995738e-1_f64 * t127799 - 0.14279934416275588154e-1_f64 * t127801 - t120003 + 0.37645955677973955999e-4_f64 * t120014 - 0.66934509195437693771e-4_f64 * t120017 - 0.17347256376410398924e1_f64 * t121975 * t27350;
    t127807
}
