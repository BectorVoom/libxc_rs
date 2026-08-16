//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1253/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1253(t102397: f64, t121076: f64, t121178: f64, t122346: f64, t122413: f64, t125749: f64, t125753: f64, t125767: f64, t14230: f64, t2022: f64, t2097: f64, t27864: f64, t27960: f64, t28888: f64, t32686: f64, t32719: f64, t34231: f64, t7274: f64, t8085: f64, t8706: f64, t8707: f64) -> f64 {
    let t128742 = 0.34271842599061411569e1_f64 * t32719 * t102397 * t2022 * t14230 - 0.22312397525430606492e-2_f64 * t125749 - 0.14874931683620404328e-2_f64 * t125753 + 0.57119737665102352616e0_f64 * t8706 * t8707 * t28888 * t2022 + 0.57119737665102352616e0_f64 * t8706 * t8707 * t8085 * t7274 - t122413 - 0.17135921299530705785e1_f64 * t34231 * t32686 + 0.57119737665102352616e0_f64 * t8706 * t8707 * t2097 * t27960 + 0.34271842599061411569e1_f64 * t121076 * t122346 * t27864 - 0.66934509195437693771e-4_f64 * t121178 - 0.34708173928447610099e-2_f64 * t125767;
    t128742
}
