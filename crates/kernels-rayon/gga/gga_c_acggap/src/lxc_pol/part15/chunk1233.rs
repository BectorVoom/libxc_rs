//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1233/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1233(t30868: f64, t30872: f64, t30880: f64, t32619: f64, t35025: f64, t35028: f64, t35030: f64, t39615: f64, t39617: f64, t39620: f64, t39623: f64, t39626: f64, t39629: f64, t39632: f64, t39640: f64, t39643: f64, t39647: f64, t39649: f64) -> f64 {
    let t41736 = 7.0_f64 / 144.0_f64 * t39615 - t39617 / 48.0_f64 - t39620 / 32.0_f64 + t39623 / 48.0_f64 + 0.305625e-1_f64 * t39626 - t39629 / 2.0_f64 + t39632 / 24.0_f64 - t35025 + 0.45351183609335988442e-1_f64 * t30868 - 0.45351183609335988442e-1_f64 * t30872 + t35028 - t35030 + 0.90035438047946447644e-2_f64 * t30880 + t32619 + 0.42874018118069736972e-3_f64 * t39640 + 0.42874018118069736972e-3_f64 * t39643 + 0.28582678745379824648e-3_f64 * t39647 - 0.17149607247227894789e-1_f64 * t39649;
    t41736
}
