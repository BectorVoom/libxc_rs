//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1370/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1370<F: Float>(t33883: F, t33941: F, t109378: F, t1596: F, t6204: F, t8335: F, t109390: F, t1591: F, t8288: F, t115105: F, t2331: F, t33781: F, t6587: F, t109756: F, t115104: F, t115913: F, t118712: F, t118715: F, t118727: F, t118733: F, t32354: F, t33937: F, t34931: F, t34950: F, t83433: F, t9536: F) -> (F, F, F, F, F) {
    let t120028 = t33941 * t33883;
    let t120036 = t6204 * t109378 * t8335 * t1596;
    let t120041 = t6204 * t109390 * t8288 * t1591;
    let t120046 = t6204 * t115105 * t8288 * t1596;
    let t120051 = t6204 * t33781 * t2331 * t6587;
    let t120060 = 0.34722222222222222222e-2 * t32354 * t34950 + 0.10722222222222222222e-1 * t109756 * t34931 - 0.11574074074074074074e-2 * t120028 + 0.31250000000000000001e-1 * t9536 * t6204 * t115913 * t83433 + 0.10416666666666666667e-1 * t9536 * t120036 + 0.116403125e-2 * t33937 * t120041 + 0.69841875000000000001e-2 * t33937 * t120046 - 0.23280625e-2 * t33937 * t120051 + 0.898632125e-3 * t115104 * t120046 - 0.23214722222222222221e-2 * t118712 - 0.51588271604938271605e-2 * t118715 - 0.11607361111111111111e-2 * t118727 + 0.11607361111111111111e-2 * t118733;
    (t120036, t120041, t120046, t120051, t120060)
}
