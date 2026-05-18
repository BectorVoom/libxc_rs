//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1406/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1406<F: Float>(t2754: F, t4482: F, t2757: F, t1057: F, t11237: F, t1052: F, t11235: F, t2751: F, t21975: F, t21978: F, t21982: F, t21984: F, t25973: F, t25975: F, t25977: F, t25980: F, t25982: F, t25984: F, t25986: F, t25990: F) -> F {
    let t30410 = t2754 * t4482;
    let t30412 = t2757 * t4482;
    let t30414 = t1057 * t11237;
    let t30416 = t1052 * t11235;
    let t30418 = t1057 * t11235;
    let t30422 = t2751 * t4482;
    let t30431 = F::new(12.0) * t30410 - t21975 - F::new(32.0) * t30412 - F::new(8.0) * t30414 + F::new(8.0) * t30416 - F::new(8.0) * t30418 - F::new(32.0) * t25973 - F::new(8.0) * t25975 + F::new(20.0) * t30422 - F::new(8.0) * t25977 + F::new(8.0) * t25980 - F::new(48.0) * t25982 - F::new(48.0) * t25984 + F::new(96.0) * t25986 - F::new(8.0) * t21978 - t21982 + t21984 + F::new(160.0) * t25990;
    t30431
}
