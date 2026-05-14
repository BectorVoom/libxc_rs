//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1064/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1064<F: Float>(t1006: F, t11003: F, t997: F, t1007: F, t4344: F, t3482: F, t9258: F, t3518: F, t9104: F, t4244: F, t967: F, t2521: F, t1410: F, t3513: F, t2478: F, t4273: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11005 = t997 * t11003 * t1006;
    let t11008 = t4344 * t1007;
    let t11016 = 4.0 * t9258 * t3482;
    let t11018 = 0.32163958997385070134e2 * t9104 * t3518;
    let t11019 = t4244 * t967;
    let t11021 = 6.0 * t2521 * t11019;
    let t11022 = t1410 * t3513;
    let t11024 = 4.0 * t2478 * t11022;
    let t11025 = t4273 * t967;
    (t11005, t11008, t11016, t11018, t11019, t11021, t11022, t11024, t11025)
}
