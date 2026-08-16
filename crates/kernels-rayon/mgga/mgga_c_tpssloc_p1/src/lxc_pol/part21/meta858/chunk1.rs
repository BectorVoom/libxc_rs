//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3116/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3116(t18918: f64, t3411: f64, t1703: f64, t51807: f64, t14858: f64, t4879: f64, t15036: f64, t4869: f64, t1155: f64, t4857: f64, t4861: f64, t51848: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64475 = 0.46785788981077169656e1_f64 * t3411 * t18918;
    let t64477 = 0.11696447245269292414e1_f64 * t51807 * t1703;
    let t64479 = 0.23392894490538584828e1_f64 * t14858 * t4879;
    let t64481 = 0.70178683471615754484e1_f64 * t4869 * t15036;
    let t64482 = t1155 * t4857;
    let t64485 = 0.4155806185363551302e3_f64 * t51848 * t4861 * t64482;
    (t64475, t64477, t64479, t64481, t64482, t64485)
}
