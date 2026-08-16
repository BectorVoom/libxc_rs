//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1300/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1300<F: Float>(t12020: F, t13917: F, t13919: F, t11360: F, t3959: F, t12210: F, t14121: F, t11426: F, t50998: F, t53447: F, t11430: F, t11363: F, t3974: F, t3990: F, t53592: F) -> (F, F, F, F, F, F) {
    let t56811 = t13917 * t13919 * t12020;
    let t56813 = t3959 * t11360;
    let t56815 = t14121 * t12210;
    let t56818 = t50998 * t53447 * t11426;
    let t56821 = t50998 * t53447 * t11430;
    let t56836 = t53592 * t3990 * t3974 * t11363;
    (t56811, t56813, t56815, t56818, t56821, t56836)
}
