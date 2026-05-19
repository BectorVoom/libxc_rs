//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 839/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk839<F: Float>(t1045: F, t7442: F, t1092: F, t2542: F, t3281: F, t7208: F, t906: F, t904: F, t3273: F, t9532: F, t9513: F, t9516: F, t9518: F, t9521: F, t9523: F, t9526: F, t9530: F, t9533: F, t9536: F) -> (F, F, F, F, F, F) {
    let t9538 = t1045 * t7442;
    let t9539 = t1092 * t9538;
    let t9541 = t2542 * t3281;
    let t9543 = t7208 * t906;
    let t9544 = t904 * t9543;
    let t9546 = t9532 * t3273;
    let t9548 = F::cast_from(0.6487109086417285278e-2_f64) * t9513 + F::cast_from(0.13900948042322754167e-2_f64) * t9516 + F::cast_from(0.27801896084645508334e-2_f64) * t9518 - F::cast_from(0.72463633678258804342e-6_f64) * t9521 - F::cast_from(0.50680539737635041234e-4_f64) * t9523 - F::cast_from(0.50680539737635041234e-4_f64) * t9526 + F::cast_from(0.151806640625e-3_f64) * t9530 - F::cast_from(0.50602213541666666668e-4_f64) * t9533 + F::cast_from(0.3373480902777777778e-5_f64) * t9536 + F::cast_from(0.14492726735651760868e-5_f64) * t9539 + F::cast_from(0.12357942809624928455e-3_f64) * t9541 + F::cast_from(0.12357942809624928455e-3_f64) * t9544 - F::cast_from(0.3373480902777777778e-5_f64) * t9546;
    (t9538, t9539, t9541, t9544, t9546, t9548)
}
