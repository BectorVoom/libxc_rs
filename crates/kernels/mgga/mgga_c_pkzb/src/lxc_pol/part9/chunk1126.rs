//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1126/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1126<F: Float>(t7716: F, t7725: F, t5984: F, t7713: F, t2064: F, t2899: F, t2902: F, t2029: F, t7575: F, t154: F, t2048: F, t276: F, t7350: F, t1137: F, t17864: F, t18331: F, t20743: F, t2104: F, t2105: F, t2106: F, t21435: F, t287: F, t2900: F, t2901: F, t2912: F, t302: F, t5537: F, t735: F, t742: F, t7632: F, t7720: F, t7742: F, t7743: F, t7857: F) -> (F, F) {
    let t21494 = t7725 * t7716;
    let t21496 = t5984 * t7713;
    let t21499 = t2899 * t2064 * t2902;
    let t21500 = 0.28582678745379824648e-3 * t21499;
    let t21518 = t7575 * t2029;
    let t21527 = t276 * t154 * t2048 * t7350;
    let t21533 = -0.43445671692977333464e-1 * t17864 * t2912 + 0.13719685797782315831e-1 * t5984 * t7720 + 0.45732285992607719436e-2 * t21494 + 0.91464571985215438873e-2 * t21496 - t21500 - 0.42874018118069736972e-3 * t2104 * t2105 * t1137 * t287 * t5537 - 0.38586616306262763275e-2 * t7742 * t302 * t21435 * t7743 + 0.42874018118069736972e-3 * t2899 * t302 * t2900 * t18331 - 0.12862205435420921092e-2 * t2104 * t2105 * t7857 * t2106 + 0.12862205435420921092e-2 * t2899 * t302 * t21518 * t2901 + t735 * t7632 / 12.0 - t21527 / 96.0 - t276 * t154 * t742 * t20743 / 96.0;
    (t21518, t21533)
}
