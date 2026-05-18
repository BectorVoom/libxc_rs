//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 936/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk936<F: Float>(t22102: F, t9816: F, t1413: F, t6816: F, t547: F, t807: F, t4011: F, t6836: F, t6871: F, t9962: F, t3930: F, t6846: F) -> (F, F, F, F, F, F, F) {
    let t22103 = t9816 * t22102;
    let t22125 = t1413 * t6816;
    let t22126 = t547 * t22125;
    let t22127 = t807 * t22126;
    let t22129 = t4011 * t6836;
    let t22130 = t547 * t22129;
    let t22131 = t807 * t22130;
    let t22156 = t9962 * t6871;
    let t22179 = t3930 * t6846;
    (t22103, t22125, t22127, t22129, t22131, t22156, t22179)
}
