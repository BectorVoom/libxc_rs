//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3872/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3872(t48786: f64, t48790: f64, t48792: f64, t48794: f64, t48796: f64, t48811: f64, t48813: f64, t74421: f64, t74425: f64, t74427: f64, t74429: f64, t74437: f64) -> f64 {
    let t74441 = -0.30492001685571196935e-2_f64 * t74421 + 0.10164000561857065645e-2_f64 * t74425 + 0.20007875121765877254e-2_f64 * t74427 + 0.13552000749142754193e-3_f64 * t74429 + 0.14291339372689912324e-4_f64 * t48786 + 0.11433071498151929859e-3_f64 * t48790 - 0.25692334753583138158e-2_f64 * t48792 + 0.45351183609335988442e0_f64 * t48794 - 0.22675591804667994222e-1_f64 * t48796 - 0.18071592998981862717e-4_f64 * t74437 - 0.80031500487063509016e-2_f64 * t48811 - 0.21683201198628406709e-2_f64 * t48813;
    t74441
}
