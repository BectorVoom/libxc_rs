//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1306/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1306<F: Float>(t1214: F, t6702: F, t3737: F, t17974: F, t5422: F, t6573: F, t1211: F, t487: F, t6564: F, t1770: F, t1811: F, t1294: F, t6744: F) -> (F, F, F, F, F, F, F) {
    let t20740 = t6702 * t1214;
    let t20741 = t3737 * t20740;
    let t20744 = t17974 * t5422;
    let t20747 = t6573 * t1214;
    let t20748 = t1211 * t20747;
    let t20753 = t6564 * t487;
    let t20756 = t1770 * t1811;
    let t20759 = t6744 * t1294;
    (t20741, t20744, t20747, t20748, t20753, t20756, t20759)
}
