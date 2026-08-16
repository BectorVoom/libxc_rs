//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 717/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk717(t1524: f64, t174: f64, t301: f64, t960: f64, t1586: f64, t372: f64, t1165: f64, t3176: f64, t4417: f64, t1150: f64, t1173: f64, t1180: f64, t335: f64, t367: f64, t3671: f64, t3673: f64, t3677: f64, t3679: f64, t3686: f64, t3694: f64, t3699: f64, t3702: f64, t3703: f64, t3733: f64, t3741: f64, t5157: f64, t5161: f64, t5165: f64, t5169: f64, t5171: f64, t5175: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5182 = t174 * t1524;
    let t5183 = t5182 * t301;
    let t5184 = t960 * t5183;
    let t5187 = t1586 * t372;
    let t5188 = t960 * t5187;
    let t5192 = t1165 * t4417 * t3176;
    let t5197 = -0.34299214494455789578e-2_f64 * t1173 * t5157 - t335 * t5161 / 24.0_f64 - t1150 * t5165 / 16.0_f64 - t5169 - t367 * t5171 / 16.0_f64 - t5175 - 0.45351183609335988442e-1_f64 * t3671 + 0.22675591804667994222e-1_f64 * t3673 - 0.22675591804667994222e-1_f64 * t3677 + 0.16006300097412701803e-1_f64 * t3679 - 0.42874018118069736972e-3_f64 * t3686 - t3694 - t3699 - t3702 + 0.12862205435420921092e-2_f64 * t3703 + t335 * t5184 / 24.0_f64 + t367 * t5188 / 24.0_f64 - 0.25724410870841842184e-2_f64 * t1180 * t5192 - 0.42874018118069736972e-3_f64 * t3733 + 0.40015750243531754508e-2_f64 * t3741;
    (t5183, t5184, t5187, t5188, t5192, t5197)
}
