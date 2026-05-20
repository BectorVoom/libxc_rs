//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2599/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2599<F: Float>(t1211: F, t20747: F, t487: F, t6564: F, t1770: F, t1811: F, t1294: F, t6744: F, t3737: F, t1248: F, t1715: F, t3604: F) -> (F, F, F, F, F) {
    let t20748 = t1211 * t20747;
    let t20753 = t6564 * t487;
    let t20756 = t1770 * t1811;
    let t20759 = t6744 * t1294;
    let t20760 = t3737 * t20759;
    let t20765 = t1715 * t1248;
    let t20766 = t3604 * t20765;
    (t20748, t20753, t20756, t20760, t20766)
}
