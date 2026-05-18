//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1137/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1137<F: Float>(t473: F, t5412: F, t13147: F, t487: F, t460: F, t12050: F, t13045: F, t13141: F, t3603: F, t1284: F, t5216: F, t1770: F, t3766: F) -> (F, F, F, F, F, F, F) {
    let t17821 = t473 * t5412;
    let t17845 = t13147 * t487;
    let t17846 = t460 * t17845;
    let t17847 = t12050 * t13045;
    let t17852 = t13141 * t487;
    let t17853 = t460 * t17852;
    let t17854 = t12050 * t3603;
    let t17861 = t5216 * t1284;
    let t17934 = t1770 * t3766;
    (t17821, t17846, t17847, t17853, t17854, t17861, t17934)
}
