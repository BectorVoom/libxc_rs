//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1136/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1136(t14019: f64, t800: f64, t5686: f64, t9744: f64, t1353: f64, t5689: f64, t1872: f64, t3889: f64, t1370: f64, t14007: f64, t14013: f64, t14016: f64, t3944: f64, t9748: f64, t9924: f64, t9926: f64, t9932: f64, t9937: f64, t9953: f64) -> f64 {
    let t14020 = t800 * t14019;
    let t14024 = 7.0_f64 / 24.0_f64 * t9744 * t5686;
    let t14026 = t800 * t5689 * t1353;
    let t14030 = t800 * t1872 * t3889;
    let t14033 = -t14007 + 0.25410001404642664112e-3_f64 * t9924 + 0.40015750243531754508e-2_f64 * t9926 + 0.71456696863449561619e-5_f64 * t9932 - 0.14291339372689912324e-4_f64 * t9937 - 0.18071592998981862717e-4_f64 * t14013 - t9748 * t14016 / 4.0_f64 - t1370 * t14020 / 48.0_f64 - t14024 + t3944 * t14026 / 8.0_f64 + t3944 * t14030 / 16.0_f64 - t9953;
    t14033
}
