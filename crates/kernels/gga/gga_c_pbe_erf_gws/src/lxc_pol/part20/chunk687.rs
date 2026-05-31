//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 687/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk687<F: Float>(t3861: F, t824: F, t905: F, t3717: F, t858: F, t886: F, t884: F, t904: F, t933: F, t2300: F, t3703: F, t3855: F) -> (F, F, F, F, F, F, F) {
    let t3862 = t3861 * t824;
    let t3863 = t905 * t3862;
    let t3866 = t858 * t3717;
    let t3867 = t886 * t3866;
    let t3869 = t884 * t3867 / F::cast_from(48.0_f64);
    let t3871 = t933 * t904 * t3717;
    let t3875 = t2300 * t904 * t3703;
    let t3879 = t858 * t3855;
    (t3862, t3863, t3867, t3869, t3871, t3875, t3879)
}
