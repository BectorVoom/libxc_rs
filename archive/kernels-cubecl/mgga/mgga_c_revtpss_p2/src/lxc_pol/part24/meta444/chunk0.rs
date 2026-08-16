//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1403/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1403<F: Float>(t1427: F, t1903: F, t22: F, t9647: F, t14296: F, t9303: F, t5718: F, t9292: F, t14099: F, t2453: F, t5603: F, t9692: F) -> (F, F, F, F, F) {
    let t47781 = t9647 * t1427 * t1903 * t22;
    let t47786 = t9303 * t14296;
    let t47802 = t9292 * t5718;
    let t47856 = t2453 * t14099;
    let t47863 = t5603 * t9692;
    (t47781, t47786, t47802, t47856, t47863)
}
