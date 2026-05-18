//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 889/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk889<F: Float>(t2585: F, t5312: F, t1648: F, t2689: F, t2566: F, t5129: F, t587: F, t2768: F, t562: F, t7694: F, t1820: F, t2620: F, t597: F) -> (F, F, F, F, F) {
    let t7710 = F::new(16.0) / F::new(45.0) * t5312 * t2585;
    let t7712 = F::new(8.0) / F::new(45.0) * t1648 * t2689;
    let t7713 = t5129 * t2566;
    let t7715 = F::new(16.0) / F::new(135.0) * t587 * t7713;
    let t7716 = t2768 * t562;
    let t7717 = t7694 * t7716;
    let t7719 = F::new(32.0) / F::new(45.0) * t1820 * t7717;
    let t7720 = t2620 * t597;
    (t7710, t7712, t7715, t7719, t7720)
}
