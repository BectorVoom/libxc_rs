//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1428/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1428<F: Float>(t22705: F, t30776: F, t412: F, t4576: F, t9691: F, t9696: F, t22954: F, t26158: F, t26409: F, t30697: F, t30760: F, t30777: F, t30781: F, t30903: F, t30908: F, t30915: F, t30919: F, t30922: F, t3733: F, t3739: F, t3747: F, t3753: F, t7806: F, t7811: F, t9575: F, t9594: F) -> (F, F, F, F) {
    let t30930 = t22705 * t412 * t30776;
    let t30933 = t4576 * t9691;
    let t30936 = t4576 * t9696;
    let t30939 = -F::new(64.0) / F::new(81.0) * t3733 * t30903 - F::new(352.0) / F::new(27.0) * t7811 * t30908 + F::new(128.0) / F::new(27.0) * t7811 * t30697 + F::new(896.0) / F::new(3.0) * t26409 * t30760 - F::new(3872.0) / F::new(729.0) * t3753 * t30915 - F::new(5600.0) / F::new(9.0) * t9575 * t30919 - F::new(28672.0) / F::new(6561.0) * t26158 * t30922 - F::new(4096.0) / F::new(729.0) * t9594 * t30777 - F::new(2560.0) / F::new(243.0) * t3739 * t30781 - F::new(1280.0) / F::new(81.0) * t3747 * t30930 + F::new(256.0) / F::new(9.0) * t22954 * t30933 + F::new(128.0) / F::new(3.0) * t7806 * t30936;
    (t30930, t30933, t30936, t30939)
}
