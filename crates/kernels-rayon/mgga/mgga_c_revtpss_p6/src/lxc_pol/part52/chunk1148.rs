//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1148/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1148(t32429: f64, t686: f64, t72: f64, t32469: f64, t32440: f64, t2061: f64, t786: f64, t25410: f64, t25413: f64, t119989: f64, t1955: f64, t2769: f64, t32433: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t121913 = t32429 * t72 * t686;
    let t121914 = t32469 * t121913;
    let t121920 = t32440 * t72 * t686;
    let t121921 = t32469 * t121920;
    let t121940 = t786 * t2061;
    let t121941 = t121940 * t25410;
    let t121942 = t121941 * t25413;
    let t121946 = 0.7052700942260554372e-3_f64 * t119989;
    let t121975 = t1955 * t32433 * t2769;
    (t121913, t121914, t121920, t121921, t121940, t121941, t121942, t121946, t121975)
}
