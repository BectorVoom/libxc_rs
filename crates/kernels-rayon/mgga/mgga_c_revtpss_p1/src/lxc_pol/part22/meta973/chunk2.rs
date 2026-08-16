//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3262/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3262(t10777: f64, t18481: f64, t50945: f64, t18333: f64, t51123: f64, t18349: f64, t2689: f64, t14494: f64, t14785: f64, t14791: f64, t14894: f64, t2745: f64, t36833: f64, t40732: f64, t4424: f64, t4433: f64, t50423: f64, t50474: f64, t50722: f64, t50724: f64, t50728: f64, t50732: f64) -> f64 {
    let t61913 = t10777 * t50945 * t18481;
    let t61916 = t10777 * t51123 * t18333;
    let t61924 = t2689 * t18349;
    let t61929 = 0.10289764348336736873e-1_f64 * t14894 * t14791 * t50474 * t50423 - 0.17149607247227894789e-1_f64 * t2745 * t14785 * t4424 * t4433 - 0.10164000561857065645e-2_f64 * t61913 + 0.2032800112371413129e-3_f64 * t61916 - 0.85748036236139473944e-3_f64 * t2745 * t36833 * t14494 * t4424 - 0.10841600599314203354e-2_f64 * t40732 - 0.16006300097412701803e-1_f64 * t50722 - 0.30488190661738479625e-3_f64 * t61924 + 0.24009450146119052705e0_f64 * t50724 - 0.57165357490759649296e-4_f64 * t50728 + 0.28582678745379824648e-4_f64 * t50732;
    t61929
}
