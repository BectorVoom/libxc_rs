//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2631/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2631<F: Float>(t1559: F, t4423: F, t2782: F, t2797: F, t14586: F, t10529: F, t18725: F, t2470: F, t2798: F, t10542: F, t18730: F, t231: F, t61749: F) -> (F, F, F, F, F) {
    let t62624 = t1559 * t4423;
    let t62626 = t2782 * t2797 * t62624;
    let t62628 = t14586 * t4423;
    let t62630 = t2782 * t10529 * t62628;
    let t62633 = t2798 * t18725 * t2470;
    let t62635 = t10542 * t18730;
    let t62637 = t61749 * t231;
    (t62626, t62630, t62633, t62635, t62637)
}
