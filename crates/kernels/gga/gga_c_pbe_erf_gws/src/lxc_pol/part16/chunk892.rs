//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 892/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk892<F: Float>(t4395: F, t8652: F, t3074: F, t2379: F, t3083: F, t2081: F, t326: F, t6469: F, t3075: F, t6472: F, t1161: F, t2416: F, t938: F, t353: F, t859: F, t1114: F, t6792: F) -> (F, F, F, F, F, F, F) {
    let t8775 = t4395 * t8652;
    let t8776 = t3074 * t8775;
    let t8780 = 7.0 / 144.0 * t3083 * t2379;
    let t8782 = t326 * t6469 * t2081;
    let t8784 = t8782 * t6472 * t3075;
    let t8787 = t2416 * t1161;
    let t8788 = t8787 * t938;
    let t8789 = t353 * t8788;
    let t8790 = t859 * t8789;
    let t8793 = t1114 * t6792;
    (t8776, t8780, t8782, t8784, t8787, t8790, t8793)
}
