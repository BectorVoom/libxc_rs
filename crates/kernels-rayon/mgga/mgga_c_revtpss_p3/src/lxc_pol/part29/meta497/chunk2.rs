//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1808/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1808(t1903: f64, t7506: f64, t7296: f64, t27924: f64, t27926: f64, t27929: f64, t25974: f64, t25980: f64, t25989: f64, t25998: f64, t26006: f64, t26025: f64, t26321: f64, t26324: f64, t26328: f64, t27919: f64, t27921: f64) -> (f64, f64, f64) {
    let t28862 = t7506 * t1903;
    let t28863 = t7296 * t28862;
    let t28872 = 0.2032800112371413129e-3_f64 * t27924;
    let t28873 = 0.16006300097412701803e-1_f64 * t27926;
    let t28874 = 0.28582678745379824648e-4_f64 * t27929;
    let t28875 = -0.50820002809285328225e-4_f64 * t25998 + t26321 + 0.40015750243531754507e-2_f64 * t26025 + t26328 - t25974 + t25980 + t25989 + 0.17149607247227894789e-1_f64 * t27919 + 0.40015750243531754507e-2_f64 * t27921 + t26006 - t26324 - t28872 + t28873 + t28874;
    (t28862, t28863, t28875)
}
