//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1187/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1187(t127365: f64, t127335: f64, t127336: f64, t127340: f64, t127341: f64, t127346: f64, t127349: f64, t127357: f64, t127359: f64, t127361: f64, t127363: f64, t1932: f64, t27830: f64, t28053: f64, t32107: f64, t32109: f64, t32112: f64, t6983: f64, t6985: f64, t7883: f64, t8463: f64) -> f64 {
    let t127366 = 2.0_f64 * t127365;
    let t127367 = -2.0_f64 * t1932 * t27830 - 4.0_f64 * t28053 * t6985 - 2.0_f64 * t6983 * t7883 + t127335 - 6.0_f64 * t127336 - t127340 + 6.0_f64 * t127341 - t127346 + t127349 - t127357 - t127359 + t127361 - 4.0_f64 * t127363 - t127366 - t32107 - t32109 - t32112 - t8463;
    t127367
}
