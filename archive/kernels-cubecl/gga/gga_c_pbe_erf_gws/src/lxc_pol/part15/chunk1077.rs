//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1077/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1077<F: Float>(t874: F, t898: F, t343: F, t938: F, t13796: F, t3989: F, t2272: F, t3975: F, t3972: F, t328: F, t922: F, t356: F) -> (F, F, F, F, F, F, F) {
    let t13797 = t898 * t874;
    let t13798 = t343 * t938;
    let t13799 = t13797 * t13798;
    let t13800 = t13796 * t13799;
    let t13801 = t3989 * t13800;
    let t13803 = t3975 * t2272;
    let t13804 = t3972 * t13803;
    let t13806 = t328 * t922;
    let t13807 = t356 * t13806;
    (t13798, t13800, t13801, t13803, t13804, t13806, t13807)
}
