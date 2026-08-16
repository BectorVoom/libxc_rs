//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 773/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk773(t1466: f64, t7822: f64, t1470: f64, t8632: f64, t8635: f64, t8638: f64, t8640: f64, t8642: f64, t8644: f64, t8646: f64, t8650: f64, t8654: f64, t8658: f64, t8662: f64, t8666: f64, t8668: f64) -> f64 {
    let t8670 = t7822 * t1466;
    let t8672 = t7822 * t1470;
    let t8674 = 0.7640625e-2_f64 * t8632 + t8635 / 32.0_f64 + t8638 / 128.0_f64 + 0.17149607247227894789e-2_f64 * t8640 - 0.85748036236139473944e-3_f64 * t8642 + 0.85748036236139473944e-3_f64 * t8644 - 0.42874018118069736972e-3_f64 * t8646 + 0.53592522647587171215e-3_f64 * t8650 + 0.32155513588552302729e-2_f64 * t8654 - 0.47172138434406228102e-3_f64 * t8658 - 0.23586069217203114051e-2_f64 * t8662 + 0.10482697429868050689e-3_f64 * t8666 + 0.17149607247227894789e-2_f64 * t8668 - 0.85748036236139473944e-3_f64 * t8670 + 0.85748036236139473944e-3_f64 * t8672;
    t8674
}
