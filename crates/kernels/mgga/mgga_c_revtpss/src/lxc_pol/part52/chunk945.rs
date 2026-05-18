//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 945/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk945<F: Float>(t27931: F, t27959: F, t225: F, t1904: F, t7242: F, t689: F, t786: F, t7911: F, t1364: F, t1398: F, t1903: F, t543: F) -> (F, F, F, F, F) {
    let t27960 = t27931 + t27959;
    let t27961 = t27960 * t225;
    let t27965 = t7242 * t1904;
    let t27966 = t689 * t27965;
    let t27968 = t786 * t7911;
    let t27969 = t27968 * t1364;
    let t27972 = t1903 * t1398 * t543;
    (t27960, t27961, t27966, t27969, t27972)
}
