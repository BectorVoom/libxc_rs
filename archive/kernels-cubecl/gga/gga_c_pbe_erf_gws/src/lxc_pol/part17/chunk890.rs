//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 890/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk890<F: Float>(t2768: F, t610: F, t7720: F, t587: F, t2646: F, t719: F, t256: F, t19: F, t2522: F, t336: F, t714: F, t1061: F, t1923: F) -> (F, F, F, F) {
    let t7721 = t2768 * t610;
    let t7722 = t7720 * t7721;
    let t7724 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t587 * t7722;
    let t7726 = t2646 * t719;
    let t7728 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7726 * t256;
    let t7729 = t2522 * t19;
    let t7730 = t7729 * t336;
    let t7732 = F::cast_from(0.12155555555555555555e0_f64) * t7730 * t714;
    let t7733 = t1061 * t1923;
    (t7724, t7728, t7732, t7733)
}
