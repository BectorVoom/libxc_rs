//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 849/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk849(t4365: f64, t6035: f64, t2747: f64, t2702: f64, t2716: f64, t2721: f64, t2739: f64, t2745: f64, t4350: f64, t4355: f64, t4357: f64, t4431: f64, t6019: f64, t6024: f64, t6030: f64, t825: f64, t851: f64) -> (f64, f64) {
    let t6036 = t4365 * t6035;
    let t6037 = t2747 * t6036;
    let t6040 = -0.21437009059034868486e-3_f64 * t825 * t6019 + 0.42874018118069736972e-3_f64 * t2721 * t6024 + t2702 + t2716 - 0.10164000561857065645e-3_f64 * t4350 + 0.14291339372689912324e-4_f64 * t4355 - 0.85748036236139473944e-3_f64 * t851 * t6030 - t2739 - 0.25410001404642664112e-4_f64 * t4431 + 0.80031500487063509015e-2_f64 * t4357 + 0.17149607247227894789e-2_f64 * t2745 * t6037;
    (t6037, t6040)
}
