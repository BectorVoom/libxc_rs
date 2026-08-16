//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2774/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2774<F: Float>(t13845: F, t13847: F, t5675: F, t73856: F, t22107: F, t9962: F, t1399: F, t22245: F, t2661: F, t3992: F, t221: F, t22287: F) -> (F, F, F, F) {
    let t74469 = t13845 * t13847 * t73856 * t5675;
    let t74471 = t9962 * t22107;
    let t74475 = t2661 * t3992 * t22245 * t1399;
    let t74477 = t221 * t22287;
    (t74469, t74471, t74475, t74477)
}
