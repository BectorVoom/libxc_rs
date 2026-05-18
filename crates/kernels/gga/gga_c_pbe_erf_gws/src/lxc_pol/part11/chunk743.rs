//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 743/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk743<F: Float>(t1161: F, t8713: F, t353: F, t4386: F, t3739: F, t6832: F, t2503: F, t3083: F, t3912: F, t4473: F, t833: F, t1143: F, t2416: F) -> (F, F, F, F, F, F, F) {
    let t12180 = t8713 * t1161;
    let t12181 = t353 * t12180;
    let t12182 = t4386 * t12181;
    let t12187 = t6832 * t3739;
    let t12195 = t3083 * t2503;
    let t12198 = t3912 * t4473;
    let t12199 = t12198 * t833;
    let t12213 = t1143 * t2416;
    (t12180, t12182, t12187, t12195, t12198, t12199, t12213)
}
