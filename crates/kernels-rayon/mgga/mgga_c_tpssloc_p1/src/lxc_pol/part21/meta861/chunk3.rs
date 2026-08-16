//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3125/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3125(t4899: f64, t6138: f64, t6144: f64, t11571: f64, t15313: f64, t15320: f64, t15376: f64, t15396: f64, t3447: f64, t4904: f64, t4919: f64, t51948: f64, t51961: f64, t51970: f64, t51980: f64, t51988: f64, t51991: f64, t51995: f64, t52040: f64) -> f64 {
    let t64644 = t4899 * t6138;
    let t64648 = t4899 * t6144;
    let t64660 = 0.55555555555555555554e-3_f64 * t3447 * t52040 * t4904 + 0.11111111111111111111e-2_f64 * t3447 * t15320 * t15313 + 0.55555555555555555554e-3_f64 * t3447 * t4919 * t51961 - 0.37037037037037037036e-3_f64 * t3447 * t64644 * t11571 - 0.37037037037037037036e-3_f64 * t3447 * t64648 * t11571 + 0.46090534979423868311e-2_f64 * t15376 * t15396 + 0.14814814814814814814e-2_f64 * t51948 - 0.24691358024691358024e-3_f64 * t51970 - 0.24691358024691358024e-3_f64 * t51980 + 0.18518518518518518518e-3_f64 * t51988 - 0.37037037037037037036e-3_f64 * t51991 - 0.11111111111111111111e-2_f64 * t51995;
    t64660
}
