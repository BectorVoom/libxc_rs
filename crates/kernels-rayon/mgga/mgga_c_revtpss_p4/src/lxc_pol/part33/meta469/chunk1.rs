//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1714/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1714(t22125: f64, t547: f64, t807: f64, t4011: f64, t6836: f64, t1353: f64, t6883: f64, t800: f64, t13832: f64, t13851: f64, t13858: f64, t22107: f64, t22111: f64, t22115: f64, t22120: f64, t3934: f64, t3944: f64, t9739: f64, t9742: f64, t9766: f64) -> (f64, f64) {
    let t22126 = t547 * t22125;
    let t22127 = t807 * t22126;
    let t22129 = t4011 * t6836;
    let t22130 = t547 * t22129;
    let t22131 = t807 * t22130;
    let t22135 = t800 * t6883 * t1353;
    let t22140 = 0.85748036236139473944e-3_f64 * t3934 * t22107 - 0.42874018118069736972e-3_f64 * t3934 * t22111 - 0.21437009059034868486e-3_f64 * t3934 * t22115 - 0.42874018118069736972e-2_f64 * t3934 * t22120 - t13832 + 0.10164000561857065645e-4_f64 * t9739 - 35.0_f64 / 216.0_f64 * t9742 + 0.28582678745379824648e-4_f64 * t22127 - 0.14291339372689912324e-3_f64 * t22131 + 0.50820002809285328224e-4_f64 * t13851 + t3944 * t22135 / 16.0_f64 - 0.90357964994909313582e-5_f64 * t13858 + 0.54208002996571016772e-3_f64 * t9766;
    (t22129, t22140)
}
