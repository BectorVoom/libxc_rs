//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2389/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2389<F: Float>(t12050: F, t3601: F, t471: F, t17710: F, t1204: F, t5462: F, t3754: F, t5219: F) -> (F, F, F, F) {
    let t17951 = t12050 * t3601 * t471;
    let t17952 = t17710 * t17951;
    let t17955 = t1204 * t5462;
    let t17958 = t5219 * t3754;
    (t17951, t17952, t17955, t17958)
}
