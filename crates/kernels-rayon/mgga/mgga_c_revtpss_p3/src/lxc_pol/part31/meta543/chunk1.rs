//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1929/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1929(t5980: f64, t7038: f64, t25237: f64, t5989: f64, t5993: f64, t7045: f64, t5985: f64, t7025: f64, t6019: f64, t6030: f64, t25254: f64, t25276: f64, t25284: f64, t27228: f64, t27230: f64, t28337: f64) -> (f64, f64) {
    let t29620 = t7038 * t5980;
    let t29623 = t25237 * t5989;
    let t29627 = t7045 * t5993;
    let t29629 = t7025 * t5985;
    let t29631 = t7038 * t6019;
    let t29633 = t7045 * t6030;
    let t29635 = t25254 + t29623 / 16.0_f64 - 0.50820002809285328226e-4_f64 * t27228 + 0.40015750243531754508e-2_f64 * t27230 + 0.85748036236139473945e-2_f64 * t29627 - t29629 / 48.0_f64 + t28337 + t25276 - t25284 - 0.42874018118069736972e-3_f64 * t29631 - 0.17149607247227894789e-2_f64 * t29633;
    (t29620, t29635)
}
