//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1223/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1223<F: Float>(t13952: F, t2210: F, t2118: F, t838: F, t1176: F, t2332: F, t903: F, t3993: F, t20091: F, t4009: F, t1180: F, t6589: F) -> (F, F, F, F, F, F) {
    let t51682 = t13952 * t2210;
    let t51717 = t2118 * t838;
    let t51818 = t1176 * t2332 * t903;
    let t51819 = t51818 * t3993;
    let t51825 = t20091 * t4009;
    let t51869 = t1176 * t6589 * t1180;
    (t51682, t51717, t51818, t51819, t51825, t51869)
}
