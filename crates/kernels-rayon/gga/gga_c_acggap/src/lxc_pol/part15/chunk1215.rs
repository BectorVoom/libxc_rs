//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1215/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1215(t34210: f64, t34211: f64, t34212: f64, t34214: f64, t34215: f64, t36972: f64, t36976: f64, t36984: f64, t36987: f64, t39080: f64, t39082: f64, t39086: f64, t39088: f64, t39092: f64, t39094: f64, t39098: f64, t39100: f64, t39107: f64) -> f64 {
    let t41479 = -0.17149607247227894789e-2_f64 * t39080 + 0.17149607247227894789e-2_f64 * t39082 - t36972 - t36976 - 0.17149607247227894789e-2_f64 * t39086 + 0.17149607247227894789e-2_f64 * t39088 - 0.14291339372689912324e-3_f64 * t39092 - 0.32012600194825403606e-1_f64 * t39094 + 0.94344276868812456204e-2_f64 * t39098 - 0.18868855373762491241e-2_f64 * t39100 - t34210 - t34211 - t34212 + t34214 - 0.12579236915841660828e-2_f64 * t34215 - t36984 - t36987 - 0.64025200389650807211e-1_f64 * t39107;
    t41479
}
