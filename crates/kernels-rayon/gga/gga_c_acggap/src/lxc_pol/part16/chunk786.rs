//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 786/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk786(t2031: f64, t507: f64, t2030: f64, t2061: f64, t2060: f64, t2314: f64, t7447: f64, t7648: f64, t7650: f64, t7652: f64, t7654: f64, t7672: f64, t8801: f64, t8804: f64, t8808: f64, t8811: f64, t8814: f64, t8818: f64, t8821: f64) -> (f64, f64, f64, f64) {
    let t8823 = t507 * t2031;
    let t8824 = t2030 * t8823;
    let t8826 = t507 * t2061;
    let t8827 = t2060 * t8826;
    let t8829 = t7447 * t2314;
    let t8834 = 0.42874018118069736972e-3_f64 * t7648 + t8801 / 128.0_f64 + t8804 / 192.0_f64 + t8808 / 16.0_f64 + t8811 / 48.0_f64 + 0.114609375e-1_f64 * t8814 + 0.114609375e-1_f64 * t8818 + 0.7640625e-2_f64 * t8821 + 0.114609375e-1_f64 * t8824 + 0.7640625e-2_f64 * t8827 - 0.420234375e-1_f64 * t8829 + 0.17149607247227894789e-2_f64 * t7650 - 0.85748036236139473944e-3_f64 * t7652 + 0.85748036236139473944e-3_f64 * t7654 + t7672;
    (t8823, t8826, t8829, t8834)
}
