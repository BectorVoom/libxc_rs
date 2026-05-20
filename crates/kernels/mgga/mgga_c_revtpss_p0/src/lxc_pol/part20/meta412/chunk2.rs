//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1524/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1524<F: Float>(t1036: F, t42860: F, t42866: F, t357: F, t42871: F, t11263: F, t3124: F, t11262: F, t3150: F, t3156: F, t3161: F, t3163: F) -> (F, F, F, F, F) {
    let t42920 = t42860 * t1036 * t42866;
    let t42921 = t42871 * t357;
    let t42926 = t3124 * t11263;
    let t42929 = t3150 * t11262 * t3156;
    let t42932 = t3161 * t11262 * t3163;
    (t42920, t42921, t42926, t42929, t42932)
}
