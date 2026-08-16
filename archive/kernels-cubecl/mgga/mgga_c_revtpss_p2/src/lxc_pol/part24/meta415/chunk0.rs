//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1360/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1360<F: Float>(t3154: F, t42871: F, t1036: F, t42860: F, t42866: F, t357: F, t11628: F, t11631: F, t3144: F, t2434: F, t246: F, t3057: F, t3316: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t42872 = t3154 * t3154;
    let t42873 = t42871 * t42872;
    let t42920 = t42860 * t1036 * t42866;
    let t42921 = t42871 * t357;
    let t42977 = t42860 * t11628 * t42866;
    let t42978 = t42871 * t11631;
    let t42984 = t42860 * t3144 * t42866;
    let t42985 = t42871 * t3154;
    let t42994 = t246 * t2434;
    let t43043 = t3057 * t3316;
    (t42872, t42873, t42920, t42921, t42977, t42978, t42984, t42985, t42994, t43043)
}
