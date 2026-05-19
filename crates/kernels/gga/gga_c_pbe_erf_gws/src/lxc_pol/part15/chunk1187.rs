//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1187/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1187<F: Float>(t898: F, t1178: F, t274: F, t938: F, t814: F, t13851: F, t9270: F, t19615: F, t353: F, t859: F, t1193: F, t745: F, param_a_c: F) -> (F, F, F, F, F, F) {
    let t51020 = t898 * param_a_c;
    let t51021 = t1178 * t51020;
    let t51022 = t274 * t938;
    let t51023 = t51022 * t814;
    let t51030 = t9270 * t13851;
    let t51042 = t859 * t353 * t19615;
    let t51053 = t353 * t1193 * t745;
    (t51020, t51021, t51023, t51030, t51042, t51053)
}
