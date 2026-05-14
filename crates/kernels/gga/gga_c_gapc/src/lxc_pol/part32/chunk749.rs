//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 749/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk749<F: Float>(t904: F, t9543: F, t3273: F, t9532: F, t9513: F, t9516: F, t9518: F, t9521: F, t9523: F, t9526: F, t9530: F, t9533: F, t9536: F, t9539: F, t9541: F, t1736: F, t3292: F) -> (F, F) {
    let t9544 = t904 * t9543;
    let t9546 = t9532 * t3273;
    let t9548 = 0.6487109086417285278e-2 * t9513 + 0.13900948042322754167e-2 * t9516 + 0.27801896084645508334e-2 * t9518 - 0.72463633678258804342e-6 * t9521 - 0.50680539737635041234e-4 * t9523 - 0.50680539737635041234e-4 * t9526 + 0.151806640625e-3 * t9530 - 0.50602213541666666668e-4 * t9533 + 0.3373480902777777778e-5 * t9536 + 0.14492726735651760868e-5 * t9539 + 0.12357942809624928455e-3 * t9541 + 0.12357942809624928455e-3 * t9544 - 0.3373480902777777778e-5 * t9546;
    let t9551 = t3292 * t1736;
    (t9548, t9551)
}
