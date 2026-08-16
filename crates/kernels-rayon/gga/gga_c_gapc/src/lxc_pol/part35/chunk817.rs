//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 817/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk817(t1045: f64, t7442: f64, t1092: f64, t2542: f64, t3281: f64, t7208: f64, t906: f64, t904: f64, t3273: f64, t9532: f64, t9513: f64, t9516: f64, t9518: f64, t9521: f64, t9523: f64, t9526: f64, t9530: f64, t9533: f64, t9536: f64) -> (f64, f64) {
    let t9538 = t1045 * t7442;
    let t9539 = t1092 * t9538;
    let t9541 = t2542 * t3281;
    let t9543 = t7208 * t906;
    let t9544 = t904 * t9543;
    let t9546 = t9532 * t3273;
    let t9548 = 0.6487109086417285278e-2_f64 * t9513 + 0.13900948042322754167e-2_f64 * t9516 + 0.27801896084645508334e-2_f64 * t9518 - 0.72463633678258804342e-6_f64 * t9521 - 0.50680539737635041234e-4_f64 * t9523 - 0.50680539737635041234e-4_f64 * t9526 + 0.151806640625e-3_f64 * t9530 - 0.50602213541666666668e-4_f64 * t9533 + 0.3373480902777777778e-5_f64 * t9536 + 0.14492726735651760868e-5_f64 * t9539 + 0.12357942809624928455e-3_f64 * t9541 + 0.12357942809624928455e-3_f64 * t9544 - 0.3373480902777777778e-5_f64 * t9546;
    (t9538, t9548)
}
