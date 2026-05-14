//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 934/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk934<F: Float>(t3717: F, t6: F, t2345: F, t875: F, t3757: F, t814: F, t9441: F, t2255: F, t11478: F, t343: F, t337: F, t2121: F, t2134: F, t6241: F, t874: F, t11514: F, t3235: F) -> (F, F, F, F, F, F, F, F) {
    let t11618 = t6 * t3717;
    let t11620 = t2345 * t11618 * t875;
    let t11623 = t3757 * t814;
    let t11624 = t9441 * t11623;
    let t11625 = t2255 * t11624;
    let t11628 = t11478 * t343;
    let t11629 = t337 * t11628;
    let t11630 = t2121 * t11629;
    let t11632 = t2134 * t11630 / 96.0;
    let t11633 = t6241 * t874;
    let t11635 = t3235 * t11514 * t11633;
    (t11618, t11620, t11624, t11625, t11628, t11632, t11633, t11635)
}
