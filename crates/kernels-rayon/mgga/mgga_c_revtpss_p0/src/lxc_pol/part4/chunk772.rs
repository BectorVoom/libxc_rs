//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 772/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk772(t1390: f64, t4057: f64, t828: f64, t1389: f64, t1408: f64, t2736: f64, t1388: f64, t1410: f64, t3970: f64, t3976: f64, t3982: f64, t3987: f64, t3990: f64, t3996: f64, t4002: f64, t4006: f64, t4014: f64, t4022: f64) -> (f64, f64, f64, f64) {
    let t4059 = t1390 * t828 * t4057;
    let t4062 = t1408 * t1389;
    let t4064 = 0.25410001404642664112e-5_f64 * t2736 * t4062;
    let t4065 = -0.85748036236139473944e-3_f64 * t1410 * t3970 - t3976 - 0.10164000561857065645e-3_f64 * t3982 + t3987 + 0.80031500487063509015e-2_f64 * t3990 + 0.14291339372689912324e-4_f64 * t3996 + 0.42874018118069736972e-3_f64 * t4002 * t4006 + 0.42874018118069736972e-2_f64 * t1410 * t4014 - 0.25410001404642664112e-4_f64 * t4022 - 0.21437009059034868486e-3_f64 * t1388 * t4059 - t4064;
    (t4059, t4062, t4064, t4065)
}
