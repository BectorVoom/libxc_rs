//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1393/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1393<F: Float>(t114720: F, t1441: F, t415: F, t1451: F, t5868: F, t33527: F, t9442: F, t2213: F, t3495: F, t3778: F, t1333: F, t33532: F, t114700: F, t114704: F, t114707: F, t114712: F, t114714: F, t114716: F, t114718: F, t2718: F, t32030: F, t32163: F, t33460: F, t6221: F) -> (F, F, F, F, F, F) {
    let t114722 = t415 * t114720 * t1441;
    let t114725 = t415 * t5868 * t1451;
    let t114728 = 0.69444444444444444446e-2 * t33527 * t9442;
    let t114733 = t415 * t2213 * t3495;
    let t114736 = t415 * t2213 * t3778;
    let t114738 = t1333 * t33532;
    let t114740 = -0.24320185185185185185e-1 * t114700 + 0.49745833333333333332e-2 * t114704 + 0.13265555555555555555e-1 * t114707 + 0.40208333333333333335e-2 * t33460 * t32030 + t114712 + t114714 - t114716 - 0.88437037037037037034e-2 * t114718 - 0.49745833333333333332e-2 * t114722 + 0.33163888888888888888e-2 * t114725 - t114728 + 0.55555555555555555558e-1 * t6221 * t32163 * t2718 - 0.88437037037037037034e-2 * t114733 - 0.55273148148148148147e-3 * t114736 - 0.58958024691358024689e-2 * t114738;
    (t114722, t114725, t114733, t114736, t114738, t114740)
}
