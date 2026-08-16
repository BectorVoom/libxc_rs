//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2630/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2630(t11583: f64, t21510: f64, t11570: f64, t15376: f64, t15382: f64, t18484: f64, t3447: f64, t44478: f64, t4919: f64, t5979: f64, t64648: f64, t64951: f64, t64969: f64, t64976: f64, t64979: f64, t64981: f64, t64988: f64, t65077: f64, t7319: f64) -> (f64, f64, f64) {
    let t73444 = t11583 * t21510;
    let t73451 = t11570 * t21510;
    let t73480 = 0.83333333333333333331e-3_f64 * t3447 * t4919 * t7319 * t5979 + 0.16666666666666666666e-2_f64 * t3447 * t4919 * t65077 - 0.81481481481481481478e-2_f64 * t64951 + 0.59259259259259259257e-2_f64 * t15376 * t18484 - 0.11111111111111111111e-2_f64 * t3447 * t64648 * t15382 - 0.3086419753086419753e-3_f64 * t44478 - 0.83333333333333333331e-3_f64 * t64969 - 0.27160493827160493826e-2_f64 * t64976 + 0.18518518518518518518e-3_f64 * t64979 + 0.44444444444444444443e-2_f64 * t64981 - 0.27777777777777777777e-3_f64 * t64988;
    (t73444, t73451, t73480)
}
