//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2758/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2758<F: Float>(t13845: F, t13847: F, t5675: F, t73731: F, t3938: F, t9816: F, t9818: F, t13848: F, t5659: F, t22159: F, t48836: F, t22120: F, t9962: F) -> (F, F, F, F, F) {
    let t73734 = t13845 * t13847 * t73731 * t5675;
    let t73738 = t9816 * t9818 * t73731 * t3938;
    let t73742 = t9816 * t13847 * t13848 * t5659;
    let t73744 = t48836 * t22159;
    let t73750 = t9962 * t22120;
    (t73734, t73738, t73742, t73744, t73750)
}
