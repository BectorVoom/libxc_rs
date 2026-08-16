//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1227/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1227(t32543: f64, t32544: f64, t32545: f64, t32557: f64, t34655: f64, t34675: f64, t34698: f64, t34718: f64, t34722: f64, t34724: f64, t34738: f64, t34743: f64, t37197: f64, t37213: f64, t37214: f64, t37216: f64, t37217: f64, t39427: f64) -> f64 {
    let t41646 = -t32543 - t32544 + t32545 - t34655 + t37197 + 0.83861579438944405517e-3_f64 * t34675 - 7.0_f64 / 72.0_f64 * t39427 - 0.85748036236139473944e-2_f64 * t34698 - t37213 - t37214 + t37216 + t37217 - 0.10289764348336736873e-1_f64 * t34718 + 0.62896184579208304136e-2_f64 * t34722 + 0.37737710747524982482e-1_f64 * t34724 + 0.51448821741683684367e-2_f64 * t34738 - t32557 - t34743;
    t41646
}
