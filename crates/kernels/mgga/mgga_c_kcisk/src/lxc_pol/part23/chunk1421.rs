//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1421/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1421<F: Float>(t32388: F, t9851: F, t33873: F, t9529: F, t33851: F, t9532: F, t2736: F, t55345: F, t79: F, t2326: F, t32440: F, t4513: F, t6204: F, t33870: F, t9512: F, t109664: F, t109703: F, t109729: F, t114113: F, t114117: F, t114131: F, t1586: F, t1589: F, t1597: F, t21886: F, t2737: F, t32380: F, t33762: F, t33767: F, t33784: F, t33830: F, t83707: F, t9519: F, t9535: F, t9536: F) -> (F, F) {
    let t115454 = t9851 * t32388;
    let t115463 = t9529 * t33873;
    let t115468 = t33851 * t9532;
    let t115471 = t55345 * t79 * t2736;
    let t115482 = t6204 * t32440 * t2326 * t4513;
    let t115489 = 0.34722222222222222222e-2 * t9512 * t33870;
    let t115491 = 0.11574074074074074074e-2 * t115454 - 0.60312500000000000001e-2 * t33767 * t32380 + 0.52083333333333333333e-2 * t2737 * t1586 * t1589 * t1597 * t21886 - 0.92592592592592592593e-2 * t115463 - 0.77382407407407407406e-3 * t114113 - 0.38691203703703703703e-3 * t114117 + 0.92592592592592592592e-2 * t109729 + 0.92592592592592592593e-2 * t115468 + 0.40208333333333333334e-2 * t115471 * t9519 - 0.40208333333333333334e-2 * t109664 * t33762 - 0.10416666666666666667e-1 * t9536 * t6204 * t33830 * t83707 - 0.52083333333333333333e-2 * t9536 * t115482 - 0.23280625e-2 * t109703 * t9535 * t33784 + t115489 + 0.19345601851851851852e-2 * t114131;
    (t115482, t115491)
}
