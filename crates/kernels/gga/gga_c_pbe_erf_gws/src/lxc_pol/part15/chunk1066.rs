//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1066/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1066<F: Float>(t13972: F, t14118: F, t13772: F, t2367: F, t4002: F, t4474: F, t13899: F, t3979: F, t13996: F, t9270: F, t1176: F, t2332: F, t903: F, t3993: F, t20091: F, t4009: F) -> (F, F, F, F, F, F, F, F) {
    let t51771 = t13972 * t14118;
    let t51781 = t2367 * t13772;
    let t51788 = t4474 * t4002;
    let t51807 = t3979 * t13899;
    let t51815 = t9270 * t13996;
    let t51818 = t1176 * t2332 * t903;
    let t51819 = t51818 * t3993;
    let t51825 = t20091 * t4009;
    (t51771, t51781, t51788, t51807, t51815, t51818, t51819, t51825)
}
