//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2930/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2930<F: Float>(t47856: F, t9676: F, t14109: F, t9680: F, t9685: F, t5603: F, t9692: F, t1904: F, t689: F, t9634: F, t1364: F, t14067: F, t786: F) -> (F, F, F, F, F) {
    let t47857 = t47856 * t9676;
    let t47860 = t9680 * t14109 * t9685;
    let t47863 = t5603 * t9692;
    let t47873 = t689 * t9634 * t1904;
    let t47876 = t786 * t14067 * t1364;
    (t47857, t47860, t47863, t47873, t47876)
}
