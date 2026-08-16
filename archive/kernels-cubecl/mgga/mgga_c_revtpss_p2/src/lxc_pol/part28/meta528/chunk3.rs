//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1963/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1963<F: Float>(t225: F, t27960: F, t1904: F, t7242: F, t689: F, t786: F, t7911: F, t1364: F, t1398: F, t1903: F, t543: F, t25931: F) -> (F, F, F, F, F, F, F) {
    let t27961 = t27960 * t225;
    let t27965 = t7242 * t1904;
    let t27966 = t689 * t27965;
    let t27968 = t786 * t7911;
    let t27969 = t27968 * t1364;
    let t27972 = t1903 * t1398 * t543;
    let t27973 = t25931 * t27972;
    (t27961, t27965, t27966, t27968, t27969, t27972, t27973)
}
