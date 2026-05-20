//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1038/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1038<F: Float>(t13147: F, t487: F, t460: F, t12050: F, t13045: F, t13141: F, t3603: F, t1770: F, t3766: F, t13126: F, t3754: F, t5219: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17845 = t13147 * t487;
    let t17846 = t460 * t17845;
    let t17847 = t12050 * t13045;
    let t17852 = t13141 * t487;
    let t17853 = t460 * t17852;
    let t17854 = t12050 * t3603;
    let t17934 = t1770 * t3766;
    let t17948 = t13126 * t487;
    let t17949 = t460 * t17948;
    let t17958 = t5219 * t3754;
    (t17845, t17846, t17847, t17852, t17853, t17854, t17934, t17948, t17949, t17958)
}
