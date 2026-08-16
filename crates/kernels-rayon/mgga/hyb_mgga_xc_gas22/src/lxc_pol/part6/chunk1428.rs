//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1428/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1428(t22705: f64, t30776: f64, t412: f64, t4576: f64, t9691: f64, t9696: f64, t22954: f64, t26158: f64, t26409: f64, t30697: f64, t30760: f64, t30777: f64, t30781: f64, t30903: f64, t30908: f64, t30915: f64, t30919: f64, t30922: f64, t3733: f64, t3739: f64, t3747: f64, t3753: f64, t7806: f64, t7811: f64, t9575: f64, t9594: f64) -> (f64, f64, f64, f64) {
    let t30930 = t22705 * t412 * t30776;
    let t30933 = t4576 * t9691;
    let t30936 = t4576 * t9696;
    let t30939 = -64.0_f64 / 81.0_f64 * t3733 * t30903 - 352.0_f64 / 27.0_f64 * t7811 * t30908 + 128.0_f64 / 27.0_f64 * t7811 * t30697 + 896.0_f64 / 3.0_f64 * t26409 * t30760 - 3872.0_f64 / 729.0_f64 * t3753 * t30915 - 5600.0_f64 / 9.0_f64 * t9575 * t30919 - 28672.0_f64 / 6561.0_f64 * t26158 * t30922 - 4096.0_f64 / 729.0_f64 * t9594 * t30777 - 2560.0_f64 / 243.0_f64 * t3739 * t30781 - 1280.0_f64 / 81.0_f64 * t3747 * t30930 + 256.0_f64 / 9.0_f64 * t22954 * t30933 + 128.0_f64 / 3.0_f64 * t7806 * t30936;
    (t30930, t30933, t30936, t30939)
}
