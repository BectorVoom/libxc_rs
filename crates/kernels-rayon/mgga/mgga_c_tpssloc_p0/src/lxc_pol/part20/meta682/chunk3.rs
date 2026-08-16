//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2577/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2577(t3447: f64, t44579: f64, t4904: f64, t11545: f64, t134: f64, t461: f64, t14726: f64, t11579: f64, t15338: f64, t4899: f64, t4928: f64, t11563: f64, t11571: f64, t11572: f64, t11575: f64, t15313: f64, t15376: f64, t15390: f64, t15395: f64, t44506: f64, t44521: f64, t44608: f64, t4908: f64, t50865: f64, t50869: f64, t50910: f64, t50924: f64, t52096: f64, t52100: f64, t52110: f64, t52122: f64, t52124: f64) -> f64 {
    let t52127 = t3447 * t44579 * t4904;
    let t52133 = t134 * t11545 * t461;
    let t52135 = t3447 * t52133 * t14726;
    let t52138 = t3447 * t15338 * t11579;
    let t52140 = t4899 * t4928;
    let t52150 = 0.28806584362139917695e-2_f64 * t3447 * t52096 * t50924 + 0.86419753086419753084e-3_f64 * t3447 * t52100 * t44506 + 0.44444444444444444445e-2_f64 * t15376 * t11563 - t52110 + 0.83333333333333333331e-3_f64 * t3447 * t11575 * t15313 - 0.16666666666666666666e-2_f64 * t3447 * t4908 * t50865 - 0.49999999999999999999e-2_f64 * t3447 * t4908 * t50869 + 0.2962962962962962963e-2_f64 * t15376 * t11572 - 0.14814814814814814815e-2_f64 * t52122 - 0.82304526748971193415e-3_f64 * t52124 + 0.27777777777777777777e-3_f64 * t52127 + 0.27777777777777777777e-3_f64 * t3447 * t44521 * t4904 - 0.86419753086419753084e-3_f64 * t52135 + 0.27777777777777777777e-3_f64 * t52138 - 0.11111111111111111111e-2_f64 * t3447 * t52140 * t11571 - 0.22222222222222222221e-2_f64 * t3447 * t15390 * t44608 - 0.25925925925925925925e-2_f64 * t3447 * t15395 * t50910;
    t52150
}
