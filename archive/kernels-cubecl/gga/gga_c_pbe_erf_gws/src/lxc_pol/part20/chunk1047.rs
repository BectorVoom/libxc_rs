//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1047/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1047<F: Float>(t1109: F, t343: F, t874: F, t2118: F, t9499: F, t824: F, t8994: F, t3038: F, t3747: F, t905: F, t1113: F, t9856: F) -> (F, F, F, F, F, F, F, F) {
    let t11744 = t1109 * t874 * t343;
    let t11745 = t2118 * t11744;
    let t11746 = t9499 * t11745;
    let t11749 = t824 * t8994;
    let t11750 = t9499 * t11749;
    let t11753 = t3038 * t3747;
    let t11754 = t905 * t11753;
    let t11757 = t1113 * t9856;
    (t11744, t11745, t11746, t11749, t11750, t11753, t11754, t11757)
}
