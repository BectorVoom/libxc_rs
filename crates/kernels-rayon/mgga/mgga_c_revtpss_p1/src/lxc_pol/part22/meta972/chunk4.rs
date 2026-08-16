//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3256/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3256(t10811: f64, t18462: f64, t18466: f64, t14872: f64, t18426: f64, t2745: f64, t2747: f64, t40507: f64, t40509: f64, t40518: f64, t40523: f64, t40526: f64, t40529: f64, t40532: f64, t40535: f64) -> f64 {
    let t61774 = t10811 * t18462;
    let t61776 = t10811 * t18466;
    let t61789 = 0.40015750243531754508e-2_f64 * t61774 + 0.20007875121765877254e-2_f64 * t61776 + 0.85748036236139473944e-3_f64 * t2745 * t2747 * t18426 * t14872 + t40507 + 0.25410001404642664112e-5_f64 * t40509 - 0.91476005056713590802e-4_f64 * t40518 - 0.50820002809285328224e-5_f64 * t40523 - 0.18071592998981862716e-4_f64 * t40526 + 0.25410001404642664112e-5_f64 * t40529 + 0.9035796499490931358e-4_f64 * t40532 + 0.65057734796334705778e-3_f64 * t40535;
    t61789
}
