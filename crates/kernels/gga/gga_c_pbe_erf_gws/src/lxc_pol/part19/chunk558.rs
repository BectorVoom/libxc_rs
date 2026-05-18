//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 558/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk558<F: Float>(t138: F, t1572: F, t1577: F, t2900: F, t2902: F, t2905: F, t2919: F, t514: F, t520: F, t985: F, t101: F, t1076: F, t169: F, t301: F, t784: F) -> (F, F, F) {
    let t2921 = t138 * t2900 - t1572 * t985 + F::new(2.0) * t1577 * t2905 - t2902 * t520 - t2919 * t514;
    let t2922 = t101 * t2921;
    let t2926 = t169 * t784 * t1076 * t301;
    (t2921, t2922, t2926)
}
