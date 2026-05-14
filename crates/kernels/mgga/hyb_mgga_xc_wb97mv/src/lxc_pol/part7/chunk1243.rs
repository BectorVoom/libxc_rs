//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1243/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1243<F: Float>(t10735: F, t1852: F, t10730: F, t10621: F, t10701: F, t10706: F, t10728: F, t10733: F, t2013: F, t2039: F, t2063: F, t2065: F, t2066: F, t21982: F, t25911: F, t29290: F, t29383: F, t30312: F, t3188: F, t3194: F, t3979: F, t3988: F, t453: F, t571: F, t6528: F, t6536: F, t674: F, t8645: F) -> (F,) {
    let t30417 = t1852 * t10735;
    let t30419 = t1852 * t10730;
    let t30442 = t571 * t3194 * t10733 * t2013 / 27.0 + 20.0 / 81.0 * t571 * t8645 * t21982 * t3979 * t2039 - 4.0 / 9.0 * t571 * t3188 * t10701 * t2039 + 2.0 / 27.0 * t571 * t3188 * t6536 * t3988 * t2039 - t571 * t3194 * t10728 * t2039 / 9.0 + 4.0 / 9.0 * t571 * t3194 * t10706 * t2039 - 2.0 / 81.0 * t30417 + 2.0 / 243.0 * t30419 - 2.0 / 81.0 * t571 * t3188 * t2065 * t10621 * t674 - t571 * t3188 * t10728 * t2013 / 81.0 - 5.0 / 243.0 * t571 * t8645 * t6528 * t3988 * t2039 + 8.0 / 9.0 * t29290 * t25911 * t30312 - 8.0 / 81.0 * t29383 * t2063 * t2066 * t453;
    (t30442,)
}
