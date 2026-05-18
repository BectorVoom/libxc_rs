//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 985/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk985<F: Float>(t16760: F, t16765: F, t16768: F, t16771: F, t16775: F, t16777: F, t16781: F, t16787: F, t16792: F, t16796: F, t16800: F, t5385: F, t708: F) -> (F, F) {
    let t18191 = t16760 + t16765 - t16768 - t16771 - t16775 - t16777 - t16781 - t16787 + t16792 + t16796 + t16800;
    let t18192 = t708 * t5385;
    (t18191, t18192)
}
