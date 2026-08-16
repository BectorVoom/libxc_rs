//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 790/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk790(t7674: f64, t8835: f64, t8839: f64, t8841: f64, t8843: f64, t8845: f64, t8847: f64, t8849: f64, t8851: f64, t8856: f64, t8860: f64, t8862: f64, t8864: f64, t8866: f64, t8870: f64) -> f64 {
    let t8872 = -t7674 + 0.20007875121765877254e-2_f64 * t8835 - 0.53592522647587171215e-3_f64 * t8839 + 0.85748036236139473944e-3_f64 * t8841 + 0.85748036236139473944e-3_f64 * t8843 + 0.85748036236139473944e-3_f64 * t8845 - 0.85748036236139473944e-3_f64 * t8847 - 0.85748036236139473944e-3_f64 * t8849 + 0.10718504529517434243e-3_f64 * t8851 + 0.10718504529517434243e-3_f64 * t8856 + 0.7145669686344956162e-4_f64 * t8860 + 0.18868855373762491241e-2_f64 * t8862 - t8864 / 96.0_f64 - t8866 / 48.0_f64 + 0.15724046144802076034e-3_f64 * t8870;
    t8872
}
