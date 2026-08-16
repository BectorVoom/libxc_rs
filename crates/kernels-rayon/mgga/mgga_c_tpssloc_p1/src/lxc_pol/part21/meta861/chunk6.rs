//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3128/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3128(t15339: f64, t15376: f64, t15419: f64, t18232: f64, t3447: f64, t11593: f64, t15317: f64, t18427: f64, t52019: f64, t52022: f64, t52038: f64, t52050: f64, t52053: f64, t52057: f64, t52061: f64, t52064: f64) -> f64 {
    let t64730 = t15376 * t15339;
    let t64733 = t3447 * t15419 * t18232;
    let t64746 = 0.59259259259259259256e-2_f64 * t15376 * t15317 - 0.987654320987654321e-3_f64 * t64730 + 0.24691358024691358024e-3_f64 * t64733 + 0.74074074074074074072e-3_f64 * t52019 - 0.49382716049382716048e-3_f64 * t52022 + 0.37037037037037037036e-3_f64 * t52038 + 0.24691358024691358024e-3_f64 * t52050 + 0.37037037037037037036e-3_f64 * t52053 + 0.49382716049382716048e-3_f64 * t52057 - 0.24691358024691358024e-3_f64 * t52061 + 0.49382716049382716048e-3_f64 * t52064 + 0.55555555555555555554e-3_f64 * t3447 * t11593 * t18427;
    t64746
}
