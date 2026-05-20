//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1569/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1569<F: Float>(t22890: F, t9962: F, t13845: F, t22841: F, t73731: F, t9818: F, t13847: F, t1883: F, t73856: F, t9816: F, t22895: F, t125: F, t22813: F) -> (F, F, F, F, F) {
    let t85516 = t9962 * t22890;
    let t85532 = t13845 * t9818 * t73731 * t22841;
    let t85543 = t9816 * t13847 * t73856 * t1883;
    let t85545 = t9962 * t22895;
    let t85548 = t125 * t22813;
    (t85516, t85532, t85543, t85545, t85548)
}
