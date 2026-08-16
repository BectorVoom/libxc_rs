//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 781/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk781(t28395: f64, t815: f64, t23097: f64, t1516: f64, t25068: f64, t5624: f64, t6621: f64, t5572: f64, t6581: f64, t23141: f64, t23144: f64, t25109: f64, t25126: f64, t25133: f64, t26644: f64, t26646: f64, t28380: f64, t28384: f64, t28386: f64, t28390: f64) -> (f64, f64, f64, f64, f64) {
    let t28396 = t815 * t28395;
    let t28397 = t23097 * t28396;
    let t28399 = t25068 * t1516;
    let t28401 = t6621 * t5624;
    let t28403 = t6581 * t5572;
    let t28405 = 0.16956557559538964159e-1_f64 * t25109 + t28380 / 192.0_f64 - 0.12111826828242117256e-2_f64 * t28384 + t28386 / 16.0_f64 + 0.84782787797694820792e-2_f64 * t28390 + 0.28260929265898273598e-2_f64 * t25126 + 0.6728792682356731809e-4_f64 * t25133 + 0.24223653656484234512e-2_f64 * t28397 + t26644 - t28399 / 192.0_f64 + 5.0_f64 / 384.0_f64 * t28401 + t26646 - t28403 / 48.0_f64 + t23141 + t23144;
    (t28397, t28399, t28401, t28403, t28405)
}
