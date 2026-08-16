//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1233/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1233(t1137: f64, t154: f64, t17864: f64, t18331: f64, t20743: f64, t2104: f64, t2105: f64, t2106: f64, t21435: f64, t21494: f64, t21496: f64, t21500: f64, t21518: f64, t21527: f64, t276: f64, t287: f64, t2899: f64, t2900: f64, t2901: f64, t2912: f64, t302: f64, t5537: f64, t5984: f64, t735: f64, t742: f64, t7632: f64, t7720: f64, t7742: f64, t7743: f64, t7857: f64) -> f64 {
    let t21533 = -0.43445671692977333464e-1_f64 * t17864 * t2912 + 0.13719685797782315831e-1_f64 * t5984 * t7720 + 0.45732285992607719436e-2_f64 * t21494 + 0.91464571985215438873e-2_f64 * t21496 - t21500 - 0.42874018118069736972e-3_f64 * t2104 * t2105 * t1137 * t287 * t5537 - 0.38586616306262763275e-2_f64 * t7742 * t302 * t21435 * t7743 + 0.42874018118069736972e-3_f64 * t2899 * t302 * t2900 * t18331 - 0.12862205435420921092e-2_f64 * t2104 * t2105 * t7857 * t2106 + 0.12862205435420921092e-2_f64 * t2899 * t302 * t21518 * t2901 + t735 * t7632 / 12.0_f64 - t21527 / 96.0_f64 - t276 * t154 * t742 * t20743 / 96.0_f64;
    t21533
}
