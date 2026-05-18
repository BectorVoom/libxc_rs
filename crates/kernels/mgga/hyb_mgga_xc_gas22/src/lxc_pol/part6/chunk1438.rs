//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1438/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1438<F: Float>(t2869: F, t4501: F, t1572: F, t1849: F, t1620: F, t3972: F, t1145: F, t11478: F, t14635: F, t14638: F, t14641: F, t14648: F, t14770: F, t14818: F, t22746: F, t22750: F, t26560: F, t2851: F, t2858: F, t2893: F, t2923: F, t30767: F, t30915: F, t30956: F, t3739: F, t4530: F, t4576: F, t7721: F, t7800: F, t9493: F) -> (F, F, F) {
    let t31237 = t4501 * t2869;
    let t31246 = t1849 * t1572;
    let t31247 = t3972 * t1620;
    let t31248 = t31246 * t31247;
    let t31271 = -F::new(320.0) / F::new(27.0) * t14818 * t2851 * t31237 - F::new(448.0) / F::new(27.0) * t14770 * t30956 - F::new(160.0) / F::new(9.0) * t7800 * t2858 * t31237 + F::new(10000.0) / F::new(27.0) * t14635 * t31248 + F::new(50000.0) / F::new(27.0) * t14638 * t31248 - F::new(90.0) * t7721 * t1145 * t4530 * t2893 + F::new(5040.0) * t26560 * t4576 * t2923 + F::new(40000.0) / F::new(9.0) * t14641 * t31248 + F::new(70000.0) / F::new(27.0) * t14648 * t31248 - F::new(224.0) * t22750 * t30767 + F::new(160.0) * t22746 * t11478 * t9493 - F::new(3872.0) / F::new(729.0) * t3739 * t30915;
    (t31247, t31248, t31271)
}
