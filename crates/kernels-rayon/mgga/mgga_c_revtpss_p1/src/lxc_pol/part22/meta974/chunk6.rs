//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3273/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3273(t10811: f64, t18639: f64, t10905: f64, t18507: f64, t10777: f64, t10779: f64, t2749: f64, t61715: f64, t18651: f64, t14787: f64, t18426: f64, t2430: f64, t2723: f64, t2745: f64, t2747: f64, t40838: f64, t4362: f64, t4514: f64, t50459: f64, t51055: f64, t51058: f64, t51060: f64, t6035: f64, t62000: f64, t62002: f64) -> f64 {
    let t62162 = t10811 * t18639;
    let t62168 = t10905 * t18507;
    let t62176 = t10777 * t10779 * t61715 * t2749;
    let t62178 = t10811 * t18651;
    let t62186 = -0.72286371995927450868e-4_f64 * t51055 + 0.10164000561857065645e-4_f64 * t51058 + 35.0_f64 / 18.0_f64 * t51060 - 0.16006300097412701803e-1_f64 * t62162 + 0.17149607247227894789e-2_f64 * t2745 * t2747 * t50459 * t6035 - 7.0_f64 / 24.0_f64 * t62168 - 0.17149607247227894789e-1_f64 * t4514 * t62000 * t62002 * t14787 + 0.10164000561857065645e-3_f64 * t62176 + 0.20007875121765877254e-2_f64 * t62178 - 0.17149607247227894789e-2_f64 * t4362 * t2747 * t18426 * t2723 * t2430 - 35.0_f64 / 216.0_f64 * t40838;
    t62186
}
